// Add missing imports and definitions back
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use log::{info, warn};

// Re-add constants
const WEIGHT_FIN_HEALTH: Decimal = dec!(0.35); // Adjusted weights
const WEIGHT_INFRA: Decimal = dec!(0.25);
const WEIGHT_EFFICIENCY: Decimal = dec!(0.20);
const WEIGHT_ACCOUNTABILITY: Decimal = dec!(0.20);

// Target for scaling revenue per capita
const TARGET_REVENUE_PER_CAPITA: Decimal = dec!(10000.0);

// Re-add ScoringInput struct
#[derive(Debug, Clone, PartialEq)]
pub struct ScoringInput {
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
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
            "Clean Audit" => AuditOutcome::Clean,
            "Financially Unqualified" => AuditOutcome::FinanciallyUnqualified,
            "Qualified Audit Opinion" => AuditOutcome::Qualified,
            "Adverse Audit Opinion" => AuditOutcome::Adverse,
            "Disclaimer Of Audit Opinion" => AuditOutcome::Disclaimer,
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

// --- Pillar Score Calculation Functions ---

/// Calculates the Revenue per Capita sub-score (0-100).
/// Higher revenue per capita generally indicates a stronger economic base.
/// The score is scaled relative to `TARGET_REVENUE_PER_CAPITA`.
///
/// # Arguments
/// * `revenue_opt` - Total municipal revenue.
/// * `population_opt` - Municipal population count (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If revenue or population is missing or population is zero.
fn calculate_rev_per_cap_subscore(revenue_opt: Option<Decimal>, population_opt: Option<u32>) -> Option<Decimal> {
    let revenue = revenue_opt?; //.filter(|r| *r >= Decimal::ZERO)?; 
    let population = population_opt.filter(|&p| p > 0)?;
    // Convert population u32 to Decimal safely
    let population_dec = Decimal::from(population);
    let rev_per_capita = revenue / population_dec;

    // Scale score relative to the target, clamp between 0 and 1
    let scaled_ratio = (rev_per_capita / TARGET_REVENUE_PER_CAPITA).clamp(Decimal::ZERO, dec!(1.0));
    let score = scaled_ratio * dec!(100.0);

    Some(clamp_score(score))
}

/// Calculates the Debt Ratio sub-score (0-100).
/// Measures total debt relative to total revenue. Lower debt ratio yields a higher score.
/// Score = (1 - Debt/Revenue) * 100.
///
/// # Arguments
/// * `debt_opt` - Total municipal debt.
/// * `revenue_opt` - Total municipal revenue (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If debt or revenue is missing, or revenue is zero.
fn calculate_debt_ratio_subscore(debt_opt: Option<Decimal>, revenue_opt: Option<Decimal>) -> Option<Decimal> {
    let debt = debt_opt?;//.filter(|d| *d >= Decimal::ZERO)?;
    // Validate revenue using pattern matching
    let revenue = match revenue_opt {
        Some(r) if r > Decimal::ZERO => Some(r),
        _ => None, // Return None if revenue is None or zero/negative
    }?; // Propagate None if revenue is invalid

    let debt_ratio = debt / revenue;

    // Inverse relationship: higher ratio means lower score.
    // A ratio of 1.0 (debt equals revenue) maps to score 0.
    // A ratio of 0.0 maps to score 100.
    let score = (dec!(1.0) - debt_ratio) * dec!(100.0);
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
    // Note: Sub-scores are already clamped between 0-100.
    let debt_ratio_score = calculate_debt_ratio_subscore(debt_opt, revenue_opt)?;

    let weighted_score = (rev_per_cap_score * WEIGHT_REV_PER_CAP) + (debt_ratio_score * WEIGHT_DEBT_RATIO);
    // No final clamp needed here as weighted average of 0-100 scores is also 0-100.
    Some(weighted_score)
}

/// Calculates Infrastructure Investment Score (0-100).
/// Measures Capital Expenditure (Capex) as a percentage of Total Expenditure.
/// Score = (Capex / Expenditure) * 100. Capex is capped at Expenditure value.
///
/// # Arguments
/// * `capex_opt` - Capital expenditure.
/// * `expenditure_opt` - Total expenditure (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If capex or expenditure is missing, or expenditure is zero.
fn calculate_infra_score(
    capex_opt: Option<Decimal>,
    expenditure_opt: Option<Decimal>,
) -> Option<Decimal> {
    let capex = capex_opt?;//.filter(|c| *c >= Decimal::ZERO)?;
    let expenditure = match expenditure_opt {
        Some(e) if e > Decimal::ZERO => Some(e),
        _ => None, // Return None if expenditure is None or zero/negative
    }?; // Propagate None if expenditure is invalid

    // Ensure capex is non-negative and does not illogically exceed total expenditure.
    let valid_capex = capex.max(Decimal::ZERO).min(expenditure);

    let ratio = valid_capex / expenditure;
    // Clamp is technically not needed if inputs are non-negative and capex <= expenditure,
    // but `clamp_score` handles potential floating point nuances & ensures bounds.
    Some(clamp_score(ratio * dec!(100.0)))
}

/// Calculates Operating Efficiency Score (0-100).
/// Based on the Operating Surplus Ratio: (Revenue - Operating Expenditure) / Revenue.
/// Operating Expenditure = Total Expenditure - Capital Expenditure.
/// A higher surplus ratio (indicating revenue exceeds operating costs) yields a higher score.
/// The ratio is linearly scaled to the 0-100 range, with a ratio of 0.0 mapping to score 50.
///
/// # Arguments
/// * `expenditure_opt` - Total municipal expenditure.
/// * `capex_opt` - Capital expenditure.
/// * `revenue_opt` - Total municipal revenue (> 0).
///
/// # Returns
/// * `Some(score)` - Score between 0 and 100 if inputs are valid.
/// * `None` - If any input is missing, or revenue is zero.
fn calculate_efficiency_score(
    expenditure_opt: Option<Decimal>,
    capex_opt: Option<Decimal>,
    revenue_opt: Option<Decimal>,
) -> Option<Decimal> {
    let expenditure = expenditure_opt?;//.filter(|e| *e >= Decimal::ZERO)?;
    let capex = capex_opt?;//.filter(|c| *c >= Decimal::ZERO)?;
    // Validate revenue using pattern matching
    let revenue = match revenue_opt {
        Some(r) if r > Decimal::ZERO => Some(r),
        _ => None, // Return None if revenue is None or zero/negative
    }?; // Propagate None if revenue is invalid

    let operating_expenditure = expenditure - capex;
    // Ensure operating expenditure is not negative (e.g., if capex > expenditure, though unlikely).
    let valid_op_exp = operating_expenditure.max(Decimal::ZERO);

    let operating_surplus = revenue - valid_op_exp;
    // Calculate surplus ratio
    let surplus_ratio = operating_surplus / revenue;

    // Scale the ratio. A ratio of 0.0 (breakeven) could map to 50.
    // A ratio of 0.2 (20% surplus) could map to 100.
    // A ratio of -0.2 (20% deficit) could map to 0.
    // Linear scaling: Score = 50 + ratio * 250
    let score = dec!(50.0) + (surplus_ratio * dec!(250.0));
    Some(clamp_score(score))
}

/// Calculates Accountability Score based on Audit Outcome.
/// Maps specific audit outcomes to scores: Clean/Unqualified (100), Qualified (50), Adverse/Disclaimer/Unknown (0).
///
/// # Arguments
/// * `outcome_str_opt` - The audit outcome as a string (case-insensitive).
///
/// # Returns
/// * `Some(score)` - Score (0, 50, or 100) based on the recognized outcome.
/// * `None` - If the outcome string is missing.
fn calculate_accountability_score(outcome_str_opt: Option<&str>) -> Option<Decimal> {
    let outcome_str = outcome_str_opt?;
    let outcome = AuditOutcome::from(outcome_str);

    match outcome {
        AuditOutcome::Clean | AuditOutcome::FinanciallyUnqualified => Some(dec!(100.0)),
        AuditOutcome::Qualified => Some(dec!(50.0)),
        AuditOutcome::Adverse | AuditOutcome::Disclaimer => Some(dec!(0.0)),
        AuditOutcome::Unknown(_) => {
            warn!("Treating Unknown audit outcome as lowest score (0).");
            Some(dec!(0.0))
        }
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
/// - Financial Health (Revenue per Capita, Debt Ratio): 35%
/// - Infrastructure Investment (Capex Ratio): 25%
/// - Operating Efficiency (Surplus Ratio): 20%
/// - Accountability (Audit Outcome): 20%
pub fn calculate_financial_score(input: &ScoringInput) -> Option<ScoreBreakdown> {
    info!("Calculating financial score with input: {:?}", input);

    // Calculate scores for each pillar. Use `?` to propagate None if any fails.
    let financial_health_score = calculate_fin_health_score(input.revenue, input.debt, input.population)?;
    let infrastructure_score = calculate_infra_score(input.capital_expenditure, input.expenditure)?;
    let efficiency_score =
        calculate_efficiency_score(input.expenditure, input.capital_expenditure, input.revenue)?;
    let accountability_score =
        calculate_accountability_score(input.audit_outcome.as_deref())?;

    // Apply weights and sum up
    let overall_score =
        (clamp_score(financial_health_score) * WEIGHT_FIN_HEALTH)
        + (infrastructure_score * WEIGHT_INFRA) // Already clamped in its function
        + (efficiency_score * WEIGHT_EFFICIENCY) // Already clamped in its function
        + (clamp_score(accountability_score) * WEIGHT_ACCOUNTABILITY);

    // Final clamping and rounding should ideally happen just before display/storage,
    // but we apply clamping here for consistency within the breakdown.
    // Rounding (e.g., .round_dp(2)) is omitted here; should be done by the caller
    // just before display or storage to avoid intermediate precision loss.
    let breakdown = ScoreBreakdown {
        overall_score: clamp_score(overall_score), // Final clamp
        financial_health_score, // Already 0-100
        infrastructure_score,   // Already 0-100
        efficiency_score,       // Already 0-100
        accountability_score,   // Already 0, 50, 100
    };

    info!("Calculated score breakdown: {:?}", breakdown);
    Some(breakdown)
}