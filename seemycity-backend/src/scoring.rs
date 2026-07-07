use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;
use log::{debug, warn};

// Pillar weights (must sum to 1.0)
const WEIGHT_FIN_HEALTH: Decimal = dec!(0.30);
const WEIGHT_INFRA: Decimal = dec!(0.25);
const WEIGHT_EFFICIENCY: Decimal = dec!(0.25);
const WEIGHT_ACCOUNTABILITY: Decimal = dec!(0.20);

// Normalization ranges, tuned against real 2023 AUDA data (see docs/prd.md scoring rubric)
const REV_PER_CAPITA_MIN: Decimal = dec!(0.0);
const REV_PER_CAPITA_MAX: Decimal = dec!(14000.0);
const DEBT_RATIO_MIN: Decimal = dec!(0.1); // Score 100 at or below this ratio
const DEBT_RATIO_MAX: Decimal = dec!(1.0); // Score 0 at or above this ratio

// Efficiency (OpEx/Revenue) thresholds: linear 100 -> 0 across [BEST, WORST],
// so break-even (ratio 1.0) lands exactly at 50.
const EFFICIENCY_RATIO_BEST: Decimal = dec!(0.85); // Score 100
const EFFICIENCY_RATIO_WORST: Decimal = dec!(1.15); // Score 0

// Define thresholds for Infrastructure Score normalization
const INFRA_RATIO_WORST: Decimal = dec!(0.00); // Score 0
const INFRA_RATIO_MID: Decimal = dec!(0.10); // Score 50
const INFRA_RATIO_BEST: Decimal = dec!(0.30); // Score 100

// Re-add ScoringInput struct
#[derive(Debug, Clone, PartialEq)]
pub struct ScoringInput {
    pub revenue: Option<Decimal>,
    pub operational_expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>,
    pub population: Option<u32>,
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

/// Calculates the Revenue per Capita sub-score (0-100).
/// Higher revenue per capita generally indicates a stronger economic base.
/// The score is normalized linearly between REV_PER_CAPITA_MIN (score 0) and REV_PER_CAPITA_MAX (score 100).
///
/// # Arguments
/// * `revenue_opt` - Total municipal revenue.
/// * `population_opt` - Municipal population count (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If revenue or population is missing or population is zero.
fn calculate_rev_per_cap_subscore(revenue_opt: Option<Decimal>, population_opt: Option<u32>) -> Option<Decimal> {
    let revenue = revenue_opt?;
    let population = population_opt.filter(|&p| p > 0)?;

    let population_dec = Decimal::from(population);
    let rev_per_capita = revenue / population_dec;

    // Normalize score linearly between MIN and MAX thresholds
    let range = REV_PER_CAPITA_MAX - REV_PER_CAPITA_MIN;
    if range <= Decimal::ZERO { // Avoid division by zero/negative range
        return Some(if rev_per_capita >= REV_PER_CAPITA_MAX { dec!(100.0) } else { Decimal::ZERO });
    }

    let normalized_value = ((rev_per_capita - REV_PER_CAPITA_MIN) / range)
        .clamp(Decimal::ZERO, dec!(1.0));
    let score = normalized_value * dec!(100.0);

    Some(clamp_score(score)) // Clamp just in case
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
/// This score aggregates the Revenue per Capita and Debt Ratio sub-scores.
///
/// # Arguments
/// * `revenue_opt` - Total municipal revenue.
/// * `debt_opt` - Total municipal debt.
/// * `population_opt` - Municipal population count.
///
/// # Returns
/// * `Some(score)` - Weighted average score if both sub-scores can be calculated.
/// * `None` - If either sub-score calculation fails due to missing inputs.
fn calculate_fin_health_score(
    revenue_opt: Option<Decimal>,
    debt_opt: Option<Decimal>,
    population_opt: Option<u32>,
) -> Option<Decimal> {
    // Weights for sub-scores within Financial Health (must sum to 1.0)
    const WEIGHT_REV_PER_CAP: Decimal = dec!(0.5);
    const WEIGHT_DEBT_RATIO: Decimal = dec!(0.5);

    let rev_per_cap_score = calculate_rev_per_cap_subscore(revenue_opt, population_opt)?;
    let debt_ratio_score = calculate_debt_ratio_subscore(debt_opt, revenue_opt)?;

    let weighted_score = (rev_per_cap_score * WEIGHT_REV_PER_CAP) + (debt_ratio_score * WEIGHT_DEBT_RATIO);
    // No final clamp needed here as weighted average of 0-100 scores is also 0-100.
    Some(weighted_score)
}

/// Calculates Infrastructure Investment Score (0-100).
/// Based on Capital Expenditure as a percentage of Total Expenditure (CapEx / (OpEx + CapEx)).
/// Normalized piecewise: Score 0 at Ratio 0.00, 50 at Ratio 0.10, 100 at Ratio >= 0.30.
/// Higher ratio generally yields a higher score.
///
/// # Arguments
/// * `operational_expenditure_opt` - Operational expenditure.
/// * `capex_opt` - Capital expenditure.
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If operational_expenditure or capex is missing, or total expenditure is zero/negative.
fn calculate_infra_score(
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

/// Calculates Accountability Score based on Audit Outcome.
/// Maps specific audit outcomes to scores (0-100) based on prd.md.
///
/// # Arguments
/// * `outcome_str_opt` - The audit outcome string from the database.
///
/// # Returns
/// * `Some(score)` - 0, 25, 50, 75, or 100 for a recognized outcome. "Outstanding"
///   (statements never submitted) earns an explicit 0.
/// * `None` - If the outcome is missing or the label is unrecognized. An unknown
///   label means *we* can't interpret it — that must not be scored as if the
///   municipality failed its audit.
fn calculate_accountability_score(outcome_str_opt: Option<&str>) -> Option<Decimal> {
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

/// Calculates the overall financial score and its per-pillar breakdown.
///
/// Each pillar is `None` when its inputs are missing or invalid; the overall
/// score is `Some` only when **all four** pillars could be computed. Partial
/// data therefore yields partial pillar scores but never a misleading overall
/// number — a NULL overall renders as "no data" (grey) on the map.
///
/// Weights:
/// - Financial Health (Revenue per Capita, Debt Ratio): 30%
/// - Infrastructure Investment (Capex Ratio): 25%
/// - Operating Efficiency (OpEx Ratio): 25%
/// - Accountability (Audit Outcome): 20%
pub fn calculate_financial_score(input: &ScoringInput) -> ScoreBreakdown {
    debug!("Calculating financial score with input: {:?}", input);

    let fin_health_score =
        calculate_fin_health_score(input.revenue, input.debt, input.population).map(round_score);
    let infra_score =
        calculate_infra_score(input.operational_expenditure, input.capital_expenditure).map(round_score);
    let efficiency_score =
        calculate_efficiency_score(input.operational_expenditure, input.revenue).map(round_score);
    let accountability_score =
        calculate_accountability_score(input.audit_outcome.as_deref()).map(round_score);

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
        // rev/capita = 14_000 (max), debt ratio = 0.1 (best),
        // opex ratio = 0.85 (best), capex ratio = 5.1M/17M = 0.30 (best).
        ScoringInput {
            revenue: Some(dec!(14_000_000)),
            operational_expenditure: Some(dec!(11_900_000)),
            capital_expenditure: Some(dec!(5_100_000)),
            debt: Some(dec!(1_400_000)),
            audit_outcome: Some("Unqualified opinion with no findings".to_string()),
            population: Some(1000),
        }
    }

    // --- Revenue per Capita sub-score ---

    #[test]
    fn rev_per_cap_midpoint_and_bounds() {
        // 7_000 per capita = half of the 0..14_000 range
        assert_eq!(
            calculate_rev_per_cap_subscore(Some(dec!(7_000_000)), Some(1000)),
            Some(dec!(50.0))
        );
        // At/above max clamps to 100
        assert_eq!(
            calculate_rev_per_cap_subscore(Some(dec!(28_000_000)), Some(1000)),
            Some(dec!(100.0))
        );
    }

    #[test]
    fn rev_per_cap_missing_or_invalid_inputs() {
        assert_eq!(calculate_rev_per_cap_subscore(None, Some(1000)), None);
        assert_eq!(calculate_rev_per_cap_subscore(Some(dec!(1_000)), None), None);
        assert_eq!(calculate_rev_per_cap_subscore(Some(dec!(1_000)), Some(0)), None);
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

    // --- Financial Health pillar ---

    #[test]
    fn fin_health_averages_subscores() {
        // rev/capita 14_000 -> 100; debt ratio 0.55 -> 50; average = 75
        assert_eq!(
            calculate_fin_health_score(Some(dec!(14_000_000)), Some(dec!(7_700_000)), Some(1000)),
            Some(dec!(75.0))
        );
    }

    #[test]
    fn fin_health_requires_both_subscores() {
        assert_eq!(calculate_fin_health_score(Some(dec!(1)), None, Some(1000)), None);
        assert_eq!(calculate_fin_health_score(Some(dec!(1)), Some(dec!(1)), None), None);
    }

    // --- Infrastructure pillar ---

    #[test]
    fn infra_piecewise_points() {
        // ratio 0.10 -> 50 (capex 1000 of total 10_000)
        assert_eq!(calculate_infra_score(Some(dec!(9_000)), Some(dec!(1_000))), Some(dec!(50.0)));
        // ratio 0.20 -> 75 (halfway between MID 0.10=50 and BEST 0.30=100)
        assert_eq!(calculate_infra_score(Some(dec!(8_000)), Some(dec!(2_000))), Some(dec!(75.0)));
        // ratio 0.30 -> 100
        assert_eq!(calculate_infra_score(Some(dec!(7_000)), Some(dec!(3_000))), Some(dec!(100.0)));
        // ratio 0 -> 0
        assert_eq!(calculate_infra_score(Some(dec!(10_000)), Some(dec!(0))), Some(dec!(0.0)));
    }

    #[test]
    fn infra_missing_or_degenerate_inputs() {
        assert_eq!(calculate_infra_score(None, Some(dec!(1))), None);
        assert_eq!(calculate_infra_score(Some(dec!(1)), None), None);
        // zero total expenditure is treated as an earned 0, not missing data
        assert_eq!(calculate_infra_score(Some(dec!(0)), Some(dec!(0))), Some(dec!(0.0)));
    }

    // --- Efficiency pillar ---

    #[test]
    fn efficiency_linear_with_breakeven_at_50() {
        let revenue = Some(dec!(1_000_000));
        // ratio 0.85 -> 100
        assert_eq!(calculate_efficiency_score(Some(dec!(850_000)), revenue), Some(dec!(100.0)));
        // break-even ratio 1.0 -> exactly 50
        assert_eq!(calculate_efficiency_score(Some(dec!(1_000_000)), revenue), Some(dec!(50.0)));
        // ratio 1.15 -> 0
        assert_eq!(calculate_efficiency_score(Some(dec!(1_150_000)), revenue), Some(dec!(0.0)));
        // beyond bounds clamp
        assert_eq!(calculate_efficiency_score(Some(dec!(500_000)), revenue), Some(dec!(100.0)));
        assert_eq!(calculate_efficiency_score(Some(dec!(2_000_000)), revenue), Some(dec!(0.0)));
    }

    #[test]
    fn efficiency_missing_or_invalid_inputs() {
        assert_eq!(calculate_efficiency_score(None, Some(dec!(1))), None);
        assert_eq!(calculate_efficiency_score(Some(dec!(1)), None), None);
        assert_eq!(calculate_efficiency_score(Some(dec!(1)), Some(Decimal::ZERO)), None);
    }

    // --- Accountability pillar ---

    #[test]
    fn accountability_maps_real_world_labels() {
        let score = |s: &str| calculate_accountability_score(Some(s));
        // PRD-style and Treasury-API-style labels, case-insensitive
        assert_eq!(score("Unqualified - No findings"), Some(dec!(100.0)));
        assert_eq!(score("Unqualified opinion with no findings"), Some(dec!(100.0)));
        assert_eq!(score("UNQUALIFIED OPINION WITH FINDINGS"), Some(dec!(75.0)));
        assert_eq!(score("Financially Unqualified Opinion"), Some(dec!(75.0)));
        assert_eq!(score("Qualified opinion"), Some(dec!(50.0)));
        assert_eq!(score("Adverse opinion"), Some(dec!(25.0)));
        assert_eq!(score("Disclaimer of opinion"), Some(dec!(25.0)));
        // Outstanding is an earned zero, not missing data
        assert_eq!(score("Outstanding"), Some(dec!(0.0)));
        assert_eq!(score("Financial statements not submitted"), Some(dec!(0.0)));
    }

    #[test]
    fn accountability_unknown_or_missing_is_none() {
        assert_eq!(calculate_accountability_score(None), None);
        assert_eq!(calculate_accountability_score(Some("Some future label")), None);
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
        // FH: rev/capita 7_000 -> 50, debt ratio 0.55 -> 50 => 50
        // Infra: capex 3M of 10M total -> ratio 0.30 => 100
        // Eff: opex 7M / rev 7M -> ratio 1.0 => 50
        // Acc: Qualified => 50
        // Overall = 50*0.30 + 100*0.25 + 50*0.25 + 50*0.20 = 62.5
        let input = ScoringInput {
            revenue: Some(dec!(7_000_000)),
            operational_expenditure: Some(dec!(7_000_000)),
            capital_expenditure: Some(dec!(3_000_000)),
            debt: Some(dec!(3_850_000)),
            audit_outcome: Some("Qualified".to_string()),
            population: Some(1000),
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
        assert_eq!(breakdown.overall_score, None, "partial data must not produce an overall score");
    }

    #[test]
    fn overall_all_missing_is_all_none() {
        // Regression: the FS163 production row was persisted as overall_score = 0
        // with every input NULL. Missing everything must yield None everywhere.
        let input = ScoringInput {
            revenue: None,
            operational_expenditure: None,
            capital_expenditure: None,
            debt: None,
            audit_outcome: None,
            population: None,
        };
        let breakdown = calculate_financial_score(&input);
        assert_eq!(breakdown.overall_score, None);
        assert_eq!(breakdown.financial_health_score, None);
        assert_eq!(breakdown.infrastructure_score, None);
        assert_eq!(breakdown.efficiency_score, None);
        assert_eq!(breakdown.accountability_score, None);
    }
}