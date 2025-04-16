// Add missing imports and definitions back
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use log::info;

// Re-add constants
static WEIGHT_FIN_HEALTH: Decimal = dec!(0.30); // Corrected weight
const WEIGHT_INFRA: Decimal = dec!(0.25);
const WEIGHT_EFFICIENCY: Decimal = dec!(0.25); // Corrected weight
const WEIGHT_ACCOUNTABILITY: Decimal = dec!(0.20);

// Define ranges for normalization
const REV_PER_CAPITA_MIN: Decimal = dec!(0.0);
const REV_PER_CAPITA_MAX: Decimal = dec!(14000.0); // Lowered from 20000
const DEBT_RATIO_MIN: Decimal = dec!(0.1); // Score 100 at or below this ratio
const DEBT_RATIO_MAX: Decimal = dec!(1.0); // Adjusted from 0.45 (previously 1.5)

// Define thresholds for Efficiency Score normalization (+/- 15%)
const EFFICIENCY_RATIO_BEST: Decimal = dec!(0.85); // Score 100
const EFFICIENCY_RATIO_MID: Decimal = dec!(1.10);   // Score 50
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

// Re-add ScoreBreakdown struct
#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBreakdown {
    pub overall_score: Decimal,
    pub financial_health_score: Decimal,
    pub infrastructure_score: Decimal,
    pub efficiency_score: Decimal,
    pub accountability_score: Decimal,
}

// Re-add AuditOutcome enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuditOutcome {
    Clean,                    // "Clean Audit"
    FinanciallyUnqualified,   // "Financially Unqualified"
    Qualified,                // "Qualified Audit Opinion"
    Adverse,                  // "Adverse Audit Opinion"
    Disclaimer,               // "Disclaimer Of Audit Opinion"
    Unknown(String),          // Catch-all for others
}

// Implement From<&str> for AuditOutcome
impl From<&str> for AuditOutcome {
    fn from(s: &str) -> Self {
        match s.trim() {
            // Match exact strings from prd.md
            "Unqualified - No findings" => AuditOutcome::Clean,
            "Unqualified - Emphasis of Matter items" => AuditOutcome::FinanciallyUnqualified, // Re-use enum variant for mapping
            "Qualified" => AuditOutcome::Qualified,
            "Adverse" => AuditOutcome::Adverse, // Split Adverse/Disclaimer
            "Disclaimer" => AuditOutcome::Disclaimer,
            "Outstanding" => AuditOutcome::Unknown(s.to_string()), // Map Outstanding explicitly
            _ => AuditOutcome::Unknown(s.to_string()), // Catch-all including NULL if passed as string?
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
/// Normalized: Score 100 if Ratio >= 0.30, 50 if Ratio == 0.15, 0 if Ratio <= 0.05.
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
/// A lower ratio yields a higher score, normalized around breakeven (Ratio 1.0 = Score 50).
/// Score = 100 if Ratio <= 0.85; Score = 0 if Ratio >= 1.15. Linear scaling between.
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

    // Pre-calculate range and slope components for clarity and precision
    let upper_range = EFFICIENCY_RATIO_MID - EFFICIENCY_RATIO_BEST; // 1.0 - 0.85 = 0.15
    let lower_range = EFFICIENCY_RATIO_WORST - EFFICIENCY_RATIO_MID; // 1.15 - 1.0 = 0.15
    let score_mid_point = dec!(50.0);
    let score_max_point = dec!(100.0);

    // Apply the refined scaling logic (+/- 15%)
    let score = if opex_ratio <= EFFICIENCY_RATIO_BEST {
        score_max_point
    } else if opex_ratio <= EFFICIENCY_RATIO_MID {
        // Linear scale from 100 (at 0.85) down to 50 (at 1.0)
        // Score = 100 - (50 / 0.15) * (ratio - 0.85)
        if upper_range > Decimal::ZERO {
            score_max_point - (score_max_point - score_mid_point) * (opex_ratio - EFFICIENCY_RATIO_BEST) / upper_range
        } else { // Avoid division by zero if BEST == MID
            score_mid_point
        }
    } else if opex_ratio < EFFICIENCY_RATIO_WORST {
        // Linear scale from 50 (at 1.0) down to 0 (at 1.15)
        // Score = 50 - (50 / 0.15) * (ratio - 1.0)
        if lower_range > Decimal::ZERO {
             score_mid_point - score_mid_point * (opex_ratio - EFFICIENCY_RATIO_MID) / lower_range
        } else { // Avoid division by zero if MID == WORST
             dec!(0.0)
        }
    } else { // opex_ratio >= EFFICIENCY_RATIO_WORST
        dec!(0.0)
    };

    Some(clamp_score(score))
}

/// Calculates Accountability Score based on Audit Outcome.
/// Maps specific audit outcomes to scores (0-100) based on prd.md.
///
/// # Arguments
/// * `outcome_str_opt` - The audit outcome string from the database.
///
/// # Returns
/// Score (0, 25, 50, 75, or 100) based on the recognized outcome. Defaults to 0 if missing/unrecognized.
fn calculate_accountability_score(outcome_str_opt: Option<&str>) -> Decimal { // Return Decimal directly, default 0
    match outcome_str_opt {
        Some(outcome_str) => {
            // Use the From trait implicitly
            match AuditOutcome::from(outcome_str) {
                AuditOutcome::Clean => dec!(100.0),
                AuditOutcome::FinanciallyUnqualified => dec!(75.0), // Maps to "Emphasis of Matter"
                AuditOutcome::Qualified => dec!(50.0),
                AuditOutcome::Adverse => dec!(25.0),
                AuditOutcome::Disclaimer => dec!(25.0), // Group Adverse and Disclaimer
                AuditOutcome::Unknown(_) => dec!(0.0), // Includes "Outstanding" and others
            }
        }
        None => dec!(0.0), // Score 0 if outcome is missing (NULL)
    }
}

// --- Main Scoring Function ---

/// Calculates the overall financial score and its breakdown based on input metrics.
///
/// Returns `Some(ScoreBreakdown)` if all necessary inputs for weighted pillars are present.
/// Returns `None` if any essential metric for a weighted pillar is missing,
/// preventing calculation of that pillar's score.
///
/// Weights:
/// - Financial Health (Revenue per Capita, Debt Ratio): 30%
/// - Infrastructure Investment (Capex Ratio): 25%
/// - Operating Efficiency (Surplus Ratio): 25%
/// - Accountability (Audit Outcome): 20%
pub fn calculate_financial_score(input: &ScoringInput) -> Option<ScoreBreakdown> {
    info!("Calculating financial score with input: {:?}", input);

    // Calculate pillar scores, default to 0.0 if calculation fails (e.g., missing data)
    let fin_health_score = calculate_fin_health_score(input.revenue, input.debt, input.population)
        .unwrap_or(Decimal::ZERO);

    // Infra score takes OpEx (input.operational_expenditure) and CapEx
    let infra_score = calculate_infra_score(input.operational_expenditure, input.capital_expenditure) // Pass OpEx (input.operational_expenditure) and CapEx
        .unwrap_or(Decimal::ZERO);

    // Efficiency score takes OpEx (input.operational_expenditure) and Revenue
    let efficiency_score = calculate_efficiency_score(input.operational_expenditure, input.revenue) // Pass OpEx (input.operational_expenditure) and Revenue
        .unwrap_or(Decimal::ZERO);

    // Accountability score calculation now returns Decimal directly, defaulting to 0
    let accountability_score = calculate_accountability_score(input.audit_outcome.as_deref());

    // Calculate weighted overall score
    let overall_score = (fin_health_score * WEIGHT_FIN_HEALTH)
        + (infra_score * WEIGHT_INFRA)
        + (efficiency_score * WEIGHT_EFFICIENCY)
        + (accountability_score * WEIGHT_ACCOUNTABILITY);

    // Clamp the final overall score just in case
    let final_overall_score = clamp_score(overall_score);

    info!(
        "Calculated scores: Overall={:.2}, FH={:.2}, Infra={:.2}, Eff={:.2}, Acc={:.2}",
        final_overall_score,
        fin_health_score,
        infra_score,
        efficiency_score,
        accountability_score
    );

    // Return the breakdown
    Some(ScoreBreakdown {
        overall_score: final_overall_score, // Use clamped score
        financial_health_score: fin_health_score,
        infrastructure_score: infra_score,
        efficiency_score: efficiency_score,
        accountability_score: accountability_score,
    })
}