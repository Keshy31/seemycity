use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;
use log::{debug, warn};

/// Version stamp persisted with every scored row. Bump on any formula or
/// anchor change: the healing pass re-derives rows whose stored version
/// differs, migrating the whole cache lazily without upstream calls.
///
/// v2 (2026-07): own-revenue share replaces revenue-per-capita in Financial
/// Health; repairs & maintenance joins Infrastructure; UIFW joins
/// Accountability; unreliable-quality figures suppress raw-derived pillars.
pub const SCORE_VERSION: i32 = 2;

// Pillar weights (must sum to 1.0)
const WEIGHT_FIN_HEALTH: Decimal = dec!(0.30);
const WEIGHT_INFRA: Decimal = dec!(0.25);
const WEIGHT_EFFICIENCY: Decimal = dec!(0.25);
const WEIGHT_ACCOUNTABILITY: Decimal = dec!(0.20);

// Normalization ranges, tuned against real AUDA data (see docs/prd.md scoring rubric)
const DEBT_RATIO_MIN: Decimal = dec!(0.1); // Score 100 at or below this ratio
const DEBT_RATIO_MAX: Decimal = dec!(1.0); // Score 0 at or above this ratio

// Own-revenue share (1 - transfers/revenue): measures self-sufficiency rather
// than urbanity (which is what revenue-per-capita mostly measured in v1).
// Anchors: fully grant-dependent (share <= 0.25) -> 0; largely self-funded
// (share >= 0.75) -> 100, linear between. Provisional hybrid anchors — review
// against the national distribution at annual calibration.
const OWN_REVENUE_SHARE_MIN: Decimal = dec!(0.25);
const OWN_REVENUE_SHARE_MAX: Decimal = dec!(0.75);

// Efficiency (OpEx/Revenue) thresholds: linear 100 -> 0 across [BEST, WORST],
// so break-even (ratio 1.0) lands exactly at 50.
const EFFICIENCY_RATIO_BEST: Decimal = dec!(0.85); // Score 100
const EFFICIENCY_RATIO_WORST: Decimal = dec!(1.15); // Score 0

// Repairs & maintenance intensity (R&M / OpEx). National Treasury's norm is 8%
// of asset value; without asset values in scope, 8% of operating spend serves
// as the provisional 100-anchor (calibrate annually).
const RM_INTENSITY_MAX: Decimal = dec!(0.08); // Score 100 at or above
// Weight of R&M within the Infrastructure pillar when reported.
const INFRA_RM_WEIGHT: Decimal = dec!(0.30);

// UIFW (unauthorised/irregular/fruitless & wasteful) as a share of OpEx:
// none -> 100, a tenth of the budget or more -> 0.
const UIFW_RATIO_WORST: Decimal = dec!(0.10);
// Weight of UIFW within the Accountability pillar when reported.
const ACC_UIFW_WEIGHT: Decimal = dec!(0.30);

// Define thresholds for Infrastructure Score normalization
const INFRA_RATIO_WORST: Decimal = dec!(0.00); // Score 0
const INFRA_RATIO_MID: Decimal = dec!(0.10); // Score 50
const INFRA_RATIO_BEST: Decimal = dec!(0.30); // Score 100

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ScoringInput {
    pub revenue: Option<Decimal>,
    pub operational_expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>,
    pub population: Option<u32>,
    /// Operational grants received (part of revenue); basis for own-revenue share.
    pub transfers_operational: Option<Decimal>,
    /// Unauthorised/irregular/fruitless & wasteful expenditure; None = not reported.
    pub uifw_expenditure: Option<Decimal>,
    /// Repairs & maintenance spend; None = not reported.
    pub repairs_maintenance: Option<Decimal>,
    /// When the data-confidence layer graded the raw figures `unreliable`,
    /// pillars derived from them are suppressed (None) rather than computed
    /// from artifacts — e.g. negative debt must not earn a perfect debt score.
    pub data_unreliable: bool,
}

/// Per-pillar scores. A pillar is `None` when its inputs were missing or invalid,
/// which is distinct from an earned score of 0. `overall_score` is `Some` only
/// when every pillar could be computed, so "no data" never masquerades as
/// "worst in the country".
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBreakdown {
    pub overall_score: Option<Decimal>,
    pub financial_health_score: Option<Decimal>,
    pub infrastructure_score: Option<Decimal>,
    pub efficiency_score: Option<Decimal>,
    pub accountability_score: Option<Decimal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Clean,                  // Unqualified, no findings
    FinanciallyUnqualified, // Unqualified with findings / emphasis of matter
    Qualified,
    Adverse,
    Disclaimer,
    Outstanding,     // Financial statements never submitted for audit — an earned worst outcome
    Unknown(String), // Unrecognized label — treated as missing data, not as a bad outcome
}

impl From<&str> for AuditOutcome {
    fn from(s: &str) -> Self {
        // The Treasury API label wording varies by year; match the variants observed
        // in real data (kept in sync with frontend auditUtils.ts) case-insensitively.
        match s.trim().to_lowercase().as_str() {
            "unqualified - no findings" | "unqualified opinion with no findings" => {
                AuditOutcome::Clean
            }
            "unqualified - emphasis of matter items"
            | "unqualified opinion with findings"
            | "financially unqualified opinion" => AuditOutcome::FinanciallyUnqualified,
            "qualified" | "qualified opinion" => AuditOutcome::Qualified,
            "adverse" | "adverse opinion" => AuditOutcome::Adverse,
            "disclaimer" | "disclaimer of opinion" | "disclaimer with findings" => {
                AuditOutcome::Disclaimer
            }
            "outstanding" | "financial statements not submitted" => AuditOutcome::Outstanding,
            _ => AuditOutcome::Unknown(s.to_string()),
        }
    }
}

// --- Utility Functions ---

/// Clamps a decimal score between 0 and 100.
///
/// # Arguments
/// * `score` - The score to clamp.
///
/// # Returns
/// The score clamped to the range [0.0, 100.0].
fn clamp_score(score: Decimal) -> Decimal {
    score.clamp(Decimal::ZERO, dec!(100.0))
}

/// Rounds a score to the 2 decimal places the numeric(5,2) score columns store,
/// using the same half-away-from-zero strategy as Postgres. Keeping in-memory
/// scores identical to persisted ones lets the handler's score-healing pass
/// compare them byte-for-byte without false mismatches.
fn round_score(score: Decimal) -> Decimal {
    score.round_dp_with_strategy(2, RoundingStrategy::MidpointAwayFromZero)
}

// --- Pillar Score Calculation Functions ---

/// Calculates the Own-Revenue sub-score (0-100): how much of the municipality's
/// revenue it raises itself, versus receives as operational grants. Measures
/// fiscal self-sufficiency (v1's revenue-per-capita mostly measured urbanity —
/// it correlated ~0 with overall health).
///
/// # Arguments
/// * `revenue_opt` - Total municipal revenue (> 0).
/// * `transfers_opt` - Operational grants received (item 2200), part of revenue.
///
/// # Returns
/// * `Some(score)` - linear from OWN_REVENUE_SHARE_MIN (0) to _MAX (100).
/// * `None` - If revenue or transfers is missing, or revenue is zero/negative.
fn calculate_own_revenue_subscore(
    revenue_opt: Option<Decimal>,
    transfers_opt: Option<Decimal>,
) -> Option<Decimal> {
    let revenue = match revenue_opt {
        Some(r) if r > Decimal::ZERO => Some(r),
        _ => None,
    }?;
    let transfers = transfers_opt?;

    let share = ((revenue - transfers) / revenue).clamp(Decimal::ZERO, Decimal::ONE);

    let range = OWN_REVENUE_SHARE_MAX - OWN_REVENUE_SHARE_MIN;
    let normalized = ((share - OWN_REVENUE_SHARE_MIN) / range).clamp(Decimal::ZERO, Decimal::ONE);
    Some(clamp_score(normalized * dec!(100.0)))
}

/// Repairs & maintenance intensity sub-score (0-100): R&M spend relative to
/// operating spend, linear to 100 at RM_INTENSITY_MAX (Treasury 8% norm proxy).
/// `None` when either figure is missing (R&M is an optional enrichment — the
/// Infrastructure pillar falls back to capex-only).
fn calculate_rm_subscore(
    rm_opt: Option<Decimal>,
    operational_expenditure_opt: Option<Decimal>,
) -> Option<Decimal> {
    let rm = rm_opt.filter(|v| *v >= Decimal::ZERO)?;
    let opex = match operational_expenditure_opt {
        Some(o) if o > Decimal::ZERO => Some(o),
        _ => None,
    }?;
    let intensity = (rm / opex).clamp(Decimal::ZERO, RM_INTENSITY_MAX);
    Some(clamp_score(intensity / RM_INTENSITY_MAX * dec!(100.0)))
}

/// UIFW sub-score (0-100): unauthorised/irregular/fruitless & wasteful spend
/// relative to operating spend. Zero UIFW scores 100; a tenth of the budget or
/// more scores 0. `None` when not reported (Accountability falls back to the
/// audit outcome alone — absence of a UIFW fact is not proof of clean hands).
fn calculate_uifw_subscore(
    uifw_opt: Option<Decimal>,
    operational_expenditure_opt: Option<Decimal>,
) -> Option<Decimal> {
    let uifw = uifw_opt.filter(|v| *v >= Decimal::ZERO)?;
    let opex = match operational_expenditure_opt {
        Some(o) if o > Decimal::ZERO => Some(o),
        _ => None,
    }?;
    let ratio = (uifw / opex).clamp(Decimal::ZERO, UIFW_RATIO_WORST);
    Some(clamp_score((Decimal::ONE - ratio / UIFW_RATIO_WORST) * dec!(100.0)))
}

/// Calculates the Debt Ratio sub-score (0-100).
/// Measures total debt relative to total revenue. Lower debt ratio yields a higher score.
/// The score is normalized linearly between DEBT_RATIO_MIN (score 100) and DEBT_RATIO_MAX (score 0).
///
/// # Arguments
/// * `debt_opt` - Total municipal debt.
/// * `revenue_opt` - Total municipal revenue (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If debt or revenue is missing, or revenue is zero/negative.
fn calculate_debt_ratio_subscore(debt_opt: Option<Decimal>, revenue_opt: Option<Decimal>) -> Option<Decimal> {
    let debt = debt_opt?;
    let revenue = match revenue_opt {
        Some(r) if r > Decimal::ZERO => Some(r),
        _ => None, // Return None if revenue is None or zero/negative
    }?;

    let debt_ratio = debt / revenue;

    // Normalize score linearly between MIN and MAX thresholds (inverted)
    let range = DEBT_RATIO_MAX - DEBT_RATIO_MIN;
    if range <= Decimal::ZERO { // Avoid division by zero/negative range
        return Some(if debt_ratio <= DEBT_RATIO_MIN { dec!(100.0) } else { Decimal::ZERO });
    }

    // Calculate normalized position within the range
    let normalized_position = ((debt_ratio - DEBT_RATIO_MIN) / range)
        .clamp(Decimal::ZERO, dec!(1.0));

    // Invert the score: higher position in range means lower score
    let score = (dec!(1.0) - normalized_position) * dec!(100.0);

    Some(clamp_score(score))
}

/// Combined Financial Health Score (weighted average of sub-scores).
/// v2: aggregates the Own-Revenue share and Debt Ratio sub-scores — structural
/// self-sufficiency plus indebtedness.
///
/// # Returns
/// * `Some(score)` - Weighted average score if both sub-scores can be calculated.
/// * `None` - If either sub-score calculation fails due to missing inputs.
fn calculate_fin_health_score(
    revenue_opt: Option<Decimal>,
    debt_opt: Option<Decimal>,
    transfers_opt: Option<Decimal>,
) -> Option<Decimal> {
    // Weights for sub-scores within Financial Health (must sum to 1.0)
    const WEIGHT_OWN_REVENUE: Decimal = dec!(0.5);
    const WEIGHT_DEBT_RATIO: Decimal = dec!(0.5);

    let own_revenue_score = calculate_own_revenue_subscore(revenue_opt, transfers_opt)?;
    let debt_ratio_score = calculate_debt_ratio_subscore(debt_opt, revenue_opt)?;

    let weighted_score =
        (own_revenue_score * WEIGHT_OWN_REVENUE) + (debt_ratio_score * WEIGHT_DEBT_RATIO);
    // No final clamp needed here as weighted average of 0-100 scores is also 0-100.
    Some(weighted_score)
}

/// Calculates Infrastructure Investment Score (0-100).
/// v2: Capital-expenditure share (piecewise: 0 at 0.00, 50 at 0.10, 100 at
/// 0.30+) blended with repairs-&-maintenance intensity when reported —
/// building new assets AND maintaining existing ones.
///
/// # Returns
/// * `Some(score)` - capex sub-score, blended 70/30 with the R&M sub-score when available.
/// * `None` - If operational_expenditure or capex is missing.
fn calculate_infra_score(
    operational_expenditure_opt: Option<Decimal>,
    capex_opt: Option<Decimal>,
    rm_opt: Option<Decimal>,
) -> Option<Decimal> {
    let capex_score = calculate_capex_subscore(operational_expenditure_opt, capex_opt)?;
    match calculate_rm_subscore(rm_opt, operational_expenditure_opt) {
        Some(rm_score) => {
            Some(capex_score * (Decimal::ONE - INFRA_RM_WEIGHT) + rm_score * INFRA_RM_WEIGHT)
        }
        // R&M is an optional enrichment: fall back to capex alone when absent.
        None => Some(capex_score),
    }
}

/// Capital Expenditure sub-score: CapEx / (OpEx + CapEx), piecewise linear.
fn calculate_capex_subscore(
    operational_expenditure_opt: Option<Decimal>,
    capex_opt: Option<Decimal>,
) -> Option<Decimal> {
    let opex = operational_expenditure_opt?;
    let capex = capex_opt?;

    let total_expenditure = opex + capex;

    if total_expenditure <= Decimal::ZERO {
        return Some(Decimal::ZERO); // Avoid division by zero/negative, score 0
    }

    // Ensure capex used is non-negative (already filtered implicitly by Option check)
    let valid_capex = capex.max(Decimal::ZERO);

    let capex_ratio = valid_capex / total_expenditure;

    // Normalize the score based on thresholds
    let score = if capex_ratio <= INFRA_RATIO_WORST {
        dec!(0.0)
    } else if capex_ratio < INFRA_RATIO_MID {
        // Linear scale from 0 (at WORST) up to 50 (at MID)
        // Calculate slope and apply formula: slope * (value - start_value)
        let range = INFRA_RATIO_MID - INFRA_RATIO_WORST;
        if range > Decimal::ZERO {
             (dec!(50.0) / range) * (capex_ratio - INFRA_RATIO_WORST)
        } else {
             dec!(0.0) // Avoid division by zero if WORST == MID
        }
    } else if capex_ratio < INFRA_RATIO_BEST {
        // Linear scale from 50 (at MID) up to 100 (at BEST)
        let score_mid_point = dec!(50.0);
        let range = INFRA_RATIO_BEST - INFRA_RATIO_MID;
        if range > Decimal::ZERO {
            score_mid_point + (dec!(100.0) - score_mid_point) * (capex_ratio - INFRA_RATIO_MID) / range
        } else {
            score_mid_point // Avoid division by zero if MID == BEST
        }
    } else { // capex_ratio >= INFRA_RATIO_BEST
        dec!(100.0)
    };

    Some(clamp_score(score))
}

/// Calculates Operating Efficiency Score (0-100).
/// Based on the Operational Expenditure Ratio: (OpEx / Revenue).
/// A lower ratio yields a higher score: linear from 100 (Ratio <= 0.85) down to
/// 0 (Ratio >= 1.15), which puts break-even (Ratio 1.0) at exactly 50.
///
/// # Arguments
/// * `operational_expenditure_opt` - Operational expenditure.
/// * `revenue_opt` - Total municipal revenue (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If operational_expenditure or revenue is missing, or revenue is zero/negative.
fn calculate_efficiency_score(
    operational_expenditure_opt: Option<Decimal>,
    revenue_opt: Option<Decimal>,
) -> Option<Decimal> {
    let opex = operational_expenditure_opt?;
    let revenue = match revenue_opt {
        Some(r) if r > Decimal::ZERO => Some(r),
        _ => None, // Return None if revenue is None or zero/negative
    }?;

    let opex_ratio = opex / revenue;

    let range = EFFICIENCY_RATIO_WORST - EFFICIENCY_RATIO_BEST;
    if range <= Decimal::ZERO {
        return Some(if opex_ratio <= EFFICIENCY_RATIO_BEST { dec!(100.0) } else { Decimal::ZERO });
    }

    let normalized_position = ((opex_ratio - EFFICIENCY_RATIO_BEST) / range)
        .clamp(Decimal::ZERO, dec!(1.0));
    let score = (dec!(1.0) - normalized_position) * dec!(100.0);

    Some(clamp_score(score))
}

/// Calculates Accountability Score.
/// v2: the Auditor-General's opinion, blended 70/30 with UIFW intensity
/// (unauthorised/irregular/fruitless & wasteful spend) when reported.
///
/// # Returns
/// * `Some(score)` - audit sub-score, blended with UIFW when available.
/// * `None` - If the audit outcome is missing or the label is unrecognized.
fn calculate_accountability_score(
    outcome_str_opt: Option<&str>,
    uifw_opt: Option<Decimal>,
    operational_expenditure_opt: Option<Decimal>,
) -> Option<Decimal> {
    let audit_score = calculate_audit_subscore(outcome_str_opt)?;
    match calculate_uifw_subscore(uifw_opt, operational_expenditure_opt) {
        Some(uifw_score) => {
            Some(audit_score * (Decimal::ONE - ACC_UIFW_WEIGHT) + uifw_score * ACC_UIFW_WEIGHT)
        }
        // Absence of a UIFW fact is not proof of clean hands — audit stands alone.
        None => Some(audit_score),
    }
}

/// Audit-outcome sub-score, mapped per prd.md.
///
/// # Returns
/// * `Some(score)` - 0, 25, 50, 75, or 100 for a recognized outcome. "Outstanding"
///   (statements never submitted) earns an explicit 0.
/// * `None` - If the outcome is missing or the label is unrecognized. An unknown
///   label means *we* can't interpret it — that must not be scored as if the
///   municipality failed its audit.
fn calculate_audit_subscore(outcome_str_opt: Option<&str>) -> Option<Decimal> {
    match AuditOutcome::from(outcome_str_opt?) {
        AuditOutcome::Clean => Some(dec!(100.0)),
        AuditOutcome::FinanciallyUnqualified => Some(dec!(75.0)),
        AuditOutcome::Qualified => Some(dec!(50.0)),
        AuditOutcome::Adverse => Some(dec!(25.0)),
        AuditOutcome::Disclaimer => Some(dec!(25.0)), // Group Adverse and Disclaimer
        AuditOutcome::Outstanding => Some(dec!(0.0)),
        AuditOutcome::Unknown(label) => {
            warn!("Unrecognized audit outcome label {label:?}; treating as missing data");
            None
        }
    }
}

// --- Main Scoring Function ---

/// Calculates the overall financial score and its per-pillar breakdown (v2 —
/// see `SCORE_VERSION`).
///
/// Each pillar is `None` when its inputs are missing or invalid; the overall
/// score is `Some` only when **all four** pillars could be computed. Partial
/// data therefore yields partial pillar scores but never a misleading overall
/// number — a NULL overall renders as "no data" (grey) on the map.
///
/// When the data-confidence layer graded the figures `unreliable`, the three
/// pillars derived from them (Financial Health, Infrastructure, Efficiency)
/// are suppressed: artifacts like negative debt must not earn perfect
/// sub-scores. The audit pillar still stands — it is the AG's own statement.
///
/// Weights:
/// - Financial Health (Own-Revenue share, Debt Ratio): 30%
/// - Infrastructure Investment (Capex Ratio + R&M intensity): 25%
/// - Operating Efficiency (OpEx Ratio): 25%
/// - Accountability (Audit Outcome + UIFW intensity): 20%
pub fn calculate_financial_score(input: &ScoringInput) -> ScoreBreakdown {
    debug!("Calculating financial score with input: {:?}", input);

    let (fin_health_score, infra_score, efficiency_score) = if input.data_unreliable {
        debug!("Raw figures graded unreliable — suppressing raw-derived pillars");
        (None, None, None)
    } else {
        (
            calculate_fin_health_score(input.revenue, input.debt, input.transfers_operational)
                .map(round_score),
            calculate_infra_score(
                input.operational_expenditure,
                input.capital_expenditure,
                input.repairs_maintenance,
            )
            .map(round_score),
            calculate_efficiency_score(input.operational_expenditure, input.revenue)
                .map(round_score),
        )
    };
    let accountability_score = calculate_accountability_score(
        input.audit_outcome.as_deref(),
        input.uifw_expenditure,
        input.operational_expenditure,
    )
    .map(round_score);

    // Overall requires every pillar; a missing pillar must not silently count as 0.
    let overall_score = match (fin_health_score, infra_score, efficiency_score, accountability_score) {
        (Some(fh), Some(infra), Some(eff), Some(acc)) => Some(round_score(clamp_score(
            fh * WEIGHT_FIN_HEALTH
                + infra * WEIGHT_INFRA
                + eff * WEIGHT_EFFICIENCY
                + acc * WEIGHT_ACCOUNTABILITY,
        ))),
        _ => {
            debug!(
                "Overall score unavailable (pillars: FH={:?}, Infra={:?}, Eff={:?}, Acc={:?})",
                fin_health_score, infra_score, efficiency_score, accountability_score
            );
            None
        }
    };

    ScoreBreakdown {
        overall_score,
        financial_health_score: fin_health_score,
        infrastructure_score: infra_score,
        efficiency_score,
        accountability_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn full_input() -> ScoringInput {
        // Chosen so every pillar lands exactly on 100:
        // own-revenue share = 1.0 (no transfers), debt ratio = 0.1 (best),
        // opex ratio = 0.85 (best), capex ratio = 5.1M/17M = 0.30 (best),
        // clean audit, zero UIFW, R&M at the 8% norm.
        ScoringInput {
            revenue: Some(dec!(14_000_000)),
            operational_expenditure: Some(dec!(11_900_000)),
            capital_expenditure: Some(dec!(5_100_000)),
            debt: Some(dec!(1_400_000)),
            audit_outcome: Some("Unqualified opinion with no findings".to_string()),
            population: Some(1000),
            transfers_operational: Some(dec!(0)),
            uifw_expenditure: Some(dec!(0)),
            repairs_maintenance: Some(dec!(952_000)), // 8% of opex
            data_unreliable: false,
        }
    }

    // --- Own-Revenue sub-score (v2) ---

    #[test]
    fn own_revenue_share_anchors() {
        let revenue = Some(dec!(1_000_000));
        // fully self-funded -> 100
        assert_eq!(calculate_own_revenue_subscore(revenue, Some(dec!(0))), Some(dec!(100.0)));
        // share 0.75 -> 100 (top anchor)
        assert_eq!(calculate_own_revenue_subscore(revenue, Some(dec!(250_000))), Some(dec!(100.0)));
        // share 0.50 -> midpoint 50
        assert_eq!(calculate_own_revenue_subscore(revenue, Some(dec!(500_000))), Some(dec!(50.0)));
        // share 0.25 -> 0 (bottom anchor)
        assert_eq!(calculate_own_revenue_subscore(revenue, Some(dec!(750_000))), Some(dec!(0.0)));
        // fully grant-dependent -> 0
        assert_eq!(calculate_own_revenue_subscore(revenue, Some(dec!(1_000_000))), Some(dec!(0.0)));
    }

    #[test]
    fn own_revenue_missing_or_invalid_inputs() {
        assert_eq!(calculate_own_revenue_subscore(None, Some(dec!(1))), None);
        assert_eq!(calculate_own_revenue_subscore(Some(dec!(1_000)), None), None);
        assert_eq!(calculate_own_revenue_subscore(Some(Decimal::ZERO), Some(dec!(0))), None);
    }

    // --- Debt Ratio sub-score ---

    #[test]
    fn debt_ratio_bounds_and_midpoint() {
        let revenue = Some(dec!(1_000_000));
        // ratio 0.1 -> 100
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(100_000)), revenue), Some(dec!(100.0)));
        // ratio 0.55 -> midpoint 50
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(550_000)), revenue), Some(dec!(50.0)));
        // ratio 1.0 -> 0
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(1_000_000)), revenue), Some(dec!(0.0)));
        // beyond worst clamps to 0
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(2_000_000)), revenue), Some(dec!(0.0)));
    }

    #[test]
    fn debt_ratio_missing_or_invalid_inputs() {
        assert_eq!(calculate_debt_ratio_subscore(None, Some(dec!(1))), None);
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(1)), None), None);
        assert_eq!(calculate_debt_ratio_subscore(Some(dec!(1)), Some(Decimal::ZERO)), None);
    }

    // --- Financial Health pillar (v2: own-revenue + debt) ---

    #[test]
    fn fin_health_averages_subscores() {
        // own-revenue share 1.0 -> 100; debt ratio 0.55 -> 50; average = 75
        assert_eq!(
            calculate_fin_health_score(Some(dec!(1_000_000)), Some(dec!(550_000)), Some(dec!(0))),
            Some(dec!(75.0))
        );
    }

    #[test]
    fn fin_health_requires_both_subscores() {
        assert_eq!(calculate_fin_health_score(Some(dec!(1)), None, Some(dec!(0))), None);
        assert_eq!(calculate_fin_health_score(Some(dec!(1)), Some(dec!(1)), None), None);
    }

    // --- Infrastructure pillar (v2: capex + optional R&M) ---

    #[test]
    fn infra_capex_piecewise_points_without_rm() {
        // ratio 0.10 -> 50 (capex 1000 of total 10_000)
        assert_eq!(
            calculate_infra_score(Some(dec!(9_000)), Some(dec!(1_000)), None),
            Some(dec!(50.0))
        );
        // ratio 0.20 -> 75 (halfway between MID 0.10=50 and BEST 0.30=100)
        assert_eq!(
            calculate_infra_score(Some(dec!(8_000)), Some(dec!(2_000)), None),
            Some(dec!(75.0))
        );
        // ratio 0.30 -> 100
        assert_eq!(
            calculate_infra_score(Some(dec!(7_000)), Some(dec!(3_000)), None),
            Some(dec!(100.0))
        );
        // ratio 0 -> 0
        assert_eq!(
            calculate_infra_score(Some(dec!(10_000)), Some(dec!(0)), None),
            Some(dec!(0.0))
        );
    }

    #[test]
    fn infra_blends_rm_when_reported() {
        // capex ratio 0.30 -> 100; R&M 4% of opex -> 50; blend 0.7*100 + 0.3*50 = 85
        assert_eq!(
            calculate_infra_score(Some(dec!(7_000)), Some(dec!(3_000)), Some(dec!(280))),
            Some(dec!(85.0))
        );
        // R&M at/above the 8% norm -> 100; blend stays 100
        assert_eq!(
            calculate_infra_score(Some(dec!(7_000)), Some(dec!(3_000)), Some(dec!(560))),
            Some(dec!(100.0))
        );
    }

    #[test]
    fn infra_missing_inputs() {
        assert_eq!(calculate_infra_score(None, Some(dec!(1)), None), None);
        assert_eq!(calculate_infra_score(Some(dec!(1)), None, None), None);
        // zero total expenditure is an earned 0, not missing data
        assert_eq!(calculate_infra_score(Some(dec!(0)), Some(dec!(0)), None), Some(dec!(0.0)));
    }

    // --- Efficiency pillar ---

    #[test]
    fn efficiency_linear_with_breakeven_at_50() {
        let revenue = Some(dec!(1_000_000));
        assert_eq!(calculate_efficiency_score(Some(dec!(850_000)), revenue), Some(dec!(100.0)));
        assert_eq!(calculate_efficiency_score(Some(dec!(1_000_000)), revenue), Some(dec!(50.0)));
        assert_eq!(calculate_efficiency_score(Some(dec!(1_150_000)), revenue), Some(dec!(0.0)));
        assert_eq!(calculate_efficiency_score(Some(dec!(500_000)), revenue), Some(dec!(100.0)));
        assert_eq!(calculate_efficiency_score(Some(dec!(2_000_000)), revenue), Some(dec!(0.0)));
    }

    #[test]
    fn efficiency_missing_or_invalid_inputs() {
        assert_eq!(calculate_efficiency_score(None, Some(dec!(1))), None);
        assert_eq!(calculate_efficiency_score(Some(dec!(1)), None), None);
        assert_eq!(calculate_efficiency_score(Some(dec!(1)), Some(Decimal::ZERO)), None);
    }

    // --- Accountability pillar (v2: audit + optional UIFW) ---

    #[test]
    fn accountability_maps_real_world_labels_audit_only() {
        let score = |s: &str| calculate_accountability_score(Some(s), None, None);
        assert_eq!(score("Unqualified - No findings"), Some(dec!(100.0)));
        assert_eq!(score("Unqualified opinion with no findings"), Some(dec!(100.0)));
        assert_eq!(score("UNQUALIFIED OPINION WITH FINDINGS"), Some(dec!(75.0)));
        assert_eq!(score("Financially Unqualified Opinion"), Some(dec!(75.0)));
        assert_eq!(score("Qualified opinion"), Some(dec!(50.0)));
        assert_eq!(score("Adverse opinion"), Some(dec!(25.0)));
        assert_eq!(score("Disclaimer of opinion"), Some(dec!(25.0)));
        assert_eq!(score("Outstanding"), Some(dec!(0.0)));
        assert_eq!(score("Financial statements not submitted"), Some(dec!(0.0)));
    }

    #[test]
    fn accountability_blends_uifw_when_reported() {
        let opex = Some(dec!(1_000_000));
        // clean audit + zero UIFW -> 100
        assert_eq!(
            calculate_accountability_score(Some("Unqualified - No findings"), Some(dec!(0)), opex),
            Some(dec!(100.0))
        );
        // clean audit + UIFW at 5% of opex (sub-score 50): 0.7*100 + 0.3*50 = 85
        assert_eq!(
            calculate_accountability_score(
                Some("Unqualified - No findings"),
                Some(dec!(50_000)),
                opex
            ),
            Some(dec!(85.0))
        );
        // qualified audit + UIFW >= 10% of opex (sub-score 0): 0.7*50 = 35
        assert_eq!(
            calculate_accountability_score(Some("Qualified"), Some(dec!(200_000)), opex),
            Some(dec!(35.0))
        );
    }

    #[test]
    fn accountability_unknown_or_missing_is_none() {
        assert_eq!(calculate_accountability_score(None, Some(dec!(0)), Some(dec!(1))), None);
        assert_eq!(
            calculate_accountability_score(Some("Some future label"), Some(dec!(0)), Some(dec!(1))),
            None
        );
    }

    // --- Overall composition ---

    #[test]
    fn overall_all_pillars_perfect() {
        let breakdown = calculate_financial_score(&full_input());
        assert_eq!(breakdown.financial_health_score, Some(dec!(100.0)));
        assert_eq!(breakdown.infrastructure_score, Some(dec!(100.0)));
        assert_eq!(breakdown.efficiency_score, Some(dec!(100.0)));
        assert_eq!(breakdown.accountability_score, Some(dec!(100.0)));
        assert_eq!(breakdown.overall_score, Some(dec!(100.0)));
    }

    #[test]
    fn overall_weighted_mix() {
        // FH: own-revenue share 0.5 -> 50, debt ratio 0.55 -> 50 => 50
        // Infra: capex 3M of 10M total -> ratio 0.30 => 100 (no R&M reported)
        // Eff: opex 7M / rev 7M -> ratio 1.0 => 50
        // Acc: Qualified, no UIFW reported => 50
        // Overall = 50*0.30 + 100*0.25 + 50*0.25 + 50*0.20 = 62.5
        let input = ScoringInput {
            revenue: Some(dec!(7_000_000)),
            operational_expenditure: Some(dec!(7_000_000)),
            capital_expenditure: Some(dec!(3_000_000)),
            debt: Some(dec!(3_850_000)),
            audit_outcome: Some("Qualified".to_string()),
            population: Some(1000),
            transfers_operational: Some(dec!(3_500_000)),
            ..Default::default()
        };
        let breakdown = calculate_financial_score(&input);
        assert_eq!(breakdown.overall_score, Some(dec!(62.5)));
    }

    #[test]
    fn overall_none_when_any_pillar_missing() {
        let mut input = full_input();
        input.debt = None; // knocks out Financial Health only
        let breakdown = calculate_financial_score(&input);
        assert_eq!(breakdown.financial_health_score, None);
        assert_eq!(breakdown.infrastructure_score, Some(dec!(100.0)));
        assert_eq!(breakdown.efficiency_score, Some(dec!(100.0)));
        assert_eq!(breakdown.accountability_score, Some(dec!(100.0)));
        assert_eq!(
            breakdown.overall_score, None,
            "partial data must not produce an overall score"
        );
    }

    #[test]
    fn overall_all_missing_is_all_none() {
        // Regression: the FS163 production row was persisted as overall_score = 0
        // with every input NULL. Missing everything must yield None everywhere.
        let breakdown = calculate_financial_score(&ScoringInput::default());
        assert_eq!(breakdown.overall_score, None);
        assert_eq!(breakdown.financial_health_score, None);
        assert_eq!(breakdown.infrastructure_score, None);
        assert_eq!(breakdown.efficiency_score, None);
        assert_eq!(breakdown.accountability_score, None);
    }

    #[test]
    fn unreliable_figures_suppress_raw_pillars_but_not_audit() {
        // The Msinga rule: negative debt must not earn a perfect debt sub-score.
        let mut input = full_input();
        input.data_unreliable = true;
        let breakdown = calculate_financial_score(&input);
        assert_eq!(breakdown.financial_health_score, None);
        assert_eq!(breakdown.infrastructure_score, None);
        assert_eq!(breakdown.efficiency_score, None);
        // The audit pillar is the AG's statement, independent of the books.
        assert_eq!(breakdown.accountability_score, Some(dec!(100.0)));
        assert_eq!(breakdown.overall_score, None);
    }
}
