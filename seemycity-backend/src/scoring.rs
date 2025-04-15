use rust_decimal::Decimal;
use rust_decimal_macros::dec; // For decimal literals
use std::cmp::{max, min}; // For clamping

// Define weights from PRD
const WEIGHT_FIN_HEALTH: Decimal = dec!(0.30);
const WEIGHT_INFRA: Decimal = dec!(0.25);
const WEIGHT_EFFICIENCY: Decimal = dec!(0.25);
const WEIGHT_ACCOUNTABILITY: Decimal = dec!(0.20);

// Placeholder structure for input metrics - adjust types as needed based on source
pub struct ScoringInput {
    pub revenue: Option<Decimal>,
    pub expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub audit_outcome: Option<String>, // Or an Enum if we define one
    pub population: Option<f32>,      // From municipalities table
}

/// Calculates the overall financial health score (0-100) based on input metrics.
/// Returns None if any essential metric for a weighted pillar is missing.
pub fn calculate_financial_score(input: &ScoringInput) -> Option<Decimal> {
    // --- 1. Calculate Pillar Sub-Scores (0-100 range ideally) ---

    let accountability_score = calculate_accountability_score(input.audit_outcome.as_deref())?;
    let infra_score = calculate_infra_score(input.capital_expenditure, input.expenditure)?;
    let efficiency_score = calculate_efficiency_score(input.expenditure, input.capital_expenditure, input.revenue)?;
    let fin_health_score = calculate_fin_health_score(input.revenue, input.debt, input.population)?;

    // --- 2. Calculate Weighted Score ---
    let score = (fin_health_score * WEIGHT_FIN_HEALTH)
        + (infra_score * WEIGHT_INFRA)
        + (efficiency_score * WEIGHT_EFFICIENCY)
        + (accountability_score * WEIGHT_ACCOUNTABILITY);

    // --- 3. Clamp Final Score to 0-100 ---
    let zero = dec!(0.0);
    let hundred = dec!(100.0);

    // Use max(0, min(100, score)) - requires Ord trait for Decimal
    let score_clamped = if score < zero {
        zero
    } else if score > hundred {
        hundred
    } else {
        score
    };

    Some(score_clamped.round_dp(2)) // Round to 2 decimal places as per DB schema
}


// --- Helper Functions for Pillar Sub-Scores ---

/// Calculates the Accountability sub-score (0, 50, or 100).
/// Returns None if outcome is None or not recognized.
fn calculate_accountability_score(audit_outcome: Option<&str>) -> Option<Decimal> {
    match audit_outcome {
        Some("Clean Audit" | "Financially Unqualified") => Some(dec!(100.0)), // Assuming these map to "Clean" intent
        Some("Qualified Audit Opinion") => Some(dec!(50.0)),
        Some("Adverse Audit Opinion" | "Disclaimer Of Audit Opinion") => Some(dec!(0.0)), // Assuming "Disclaimer" is also poor
        Some(_) => Some(dec!(0.0)), // Default for other unrecognized non-empty strings? Or None? Let's default to 0 for now.
        None => None, // Cannot score if outcome is missing
    }
}

/// Calculates Infrastructure Investment sub-score (Capex % of Total Ex).
/// Returns None if capex or expenditure is None, or if expenditure is zero/negative.
fn calculate_infra_score(capex_opt: Option<Decimal>, expenditure_opt: Option<Decimal>) -> Option<Decimal> {
    match (capex_opt, expenditure_opt) {
        (Some(capex), Some(expenditure)) if expenditure > dec!(0.0) => {
            let ratio = capex / expenditure;
            // Clamp ratio between 0 and 1 before scaling to 0-100
             let score = max(dec!(0.0), min(dec!(1.0), ratio)) * dec!(100.0);
             Some(score)
        }
        _ => None, // Cannot calculate if data missing or invalid denominator
    }
}

/// Calculates Efficiency sub-score (Lower OpEx/Revenue is better).
/// OpEx = Expenditure - Capex. Score = max(0, 1 - (OpEx / Revenue)) * 100
/// Returns None if necessary inputs are missing or revenue is zero/negative.
fn calculate_efficiency_score(expenditure_opt: Option<Decimal>, capex_opt: Option<Decimal>, revenue_opt: Option<Decimal>) -> Option<Decimal> {
     match (expenditure_opt, capex_opt, revenue_opt) {
         (Some(expenditure), Some(capex), Some(revenue)) if revenue > dec!(0.0) => {
             let op_ex = expenditure - capex;
             // Handle case where OpEx might be negative if Capex > Expenditure (unlikely but possible)
             if op_ex < dec!(0.0) { return Some(dec!(100.0)); } // Perfect efficiency if OpEx is negative?

             let ratio = op_ex / revenue;
             // Score = (1 - ratio) * 100, clamped between 0 and 100
             let score = max(dec!(0.0), min(dec!(1.0), dec!(1.0) - ratio)) * dec!(100.0);
             Some(score)
         }
         _ => None,
     }
}

/// Calculates Financial Health sub-score (Average of RevPerCap and Debt sub-scores).
/// Returns None if either sub-score cannot be calculated.
fn calculate_fin_health_score(revenue_opt: Option<Decimal>, debt_opt: Option<Decimal>, population_opt: Option<f32>) -> Option<Decimal> {
    let rev_per_cap_subscore = calculate_rev_per_cap_subscore(revenue_opt, population_opt)?;
    let debt_subscore = calculate_debt_subscore(debt_opt, revenue_opt)?;

    Some((rev_per_cap_subscore + debt_subscore) / dec!(2.0))
}


// --- Sub-Components for Financial Health ---

/// Calculates Debt sub-score (Lower Debt/Revenue is better).
/// Score = max(0, 1 - (Debt / Revenue)) * 100
/// Returns None if inputs are None or revenue is zero/negative.
fn calculate_debt_subscore(debt_opt: Option<Decimal>, revenue_opt: Option<Decimal>) -> Option<Decimal> {
     match (debt_opt, revenue_opt) {
         (Some(debt), Some(revenue)) if revenue > dec!(0.0) => {
             let ratio = debt / revenue;
             // Score = (1 - ratio) * 100, clamped between 0 and 100
             let score = max(dec!(0.0), min(dec!(1.0), dec!(1.0) - ratio)) * dec!(100.0);
             Some(score)
         }
         _ => None,
     }
}

/// Calculates Revenue Per Capita sub-score (Higher is better).
/// Uses arbitrary scaling: Max R10k/capita = 100 points.
/// Score = min(100, (Revenue / Population) / 10000 * 100)
/// Returns None if inputs are None or population is zero/negative.
/// TODO: Revisit this arbitrary scaling - needs proper normalization based on dataset distribution.
fn calculate_rev_per_cap_subscore(revenue_opt: Option<Decimal>, population_opt: Option<f32>) -> Option<Decimal> {
     match (revenue_opt, population_opt) {
         (Some(revenue), Some(population)) if population > 0.0 => {
             // Convert f32 population to Decimal safely
             let population_dec = Decimal::from_f32_retain(population)?; // Returns None if population is NaN/Infinity

             if population_dec <= dec!(0.0) { return None; } // Double check after conversion

             let rev_per_capita = revenue / population_dec;
             let scaling_factor = dec!(10000.0); // Arbitrary max target for R/capita

             let score = min(dec!(1.0), rev_per_capita / scaling_factor) * dec!(100.0);
             Some(max(dec!(0.0), score)) // Ensure score is not negative
         }
         _ => None,
     }
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_basic_score_calculation() {
        let input = ScoringInput {
            revenue: Some(dec!(1000.0)),
            expenditure: Some(dec!(800.0)),
            capital_expenditure: Some(dec!(200.0)), // Infra = 200/800 = 0.25 -> 25 score
            debt: Some(dec!(500.0)),              // Debt = 500/1000 = 0.5 -> 50 subscore
            audit_outcome: Some("Clean Audit".to_string()), // Accountability = 100 score
            population: Some(100.0),             // RevPerCap = 1000/100 = 10. Scale: (10/10000)*100 = 0.1 subscore (Needs review!)
        };
        // FinHealth = (0.1 + 50) / 2 = 25.05
        // Efficiency = (800-200)/1000 = 0.6. Score = (1-0.6)*100 = 40 score

        // Total = (25.05 * 0.3) + (25.0 * 0.25) + (40.0 * 0.25) + (100.0 * 0.20)
        //       = 7.515       + 6.25        + 10.0        + 20.0
        //       = 43.765
        // Rounded = 43.77

        let score = calculate_financial_score(&input);
        assert!(score.is_some());
        assert_eq!(score.unwrap(), dec!(43.77));
    }

     #[test]
    fn test_missing_data_leads_to_none() {
         let input = ScoringInput {
            revenue: None, // Missing essential data for multiple pillars
            expenditure: Some(dec!(800.0)),
            capital_expenditure: Some(dec!(200.0)),
            debt: Some(dec!(500.0)),
            audit_outcome: Some("Clean Audit".to_string()),
            population: Some(100.0),
        };
         let score = calculate_financial_score(&input);
        assert!(score.is_none());
    }

     #[test]
    fn test_audit_outcomes() {
        assert_eq!(calculate_accountability_score(Some("Clean Audit")), Some(dec!(100.0)));
        assert_eq!(calculate_accountability_score(Some("Financially Unqualified")), Some(dec!(100.0)));
        assert_eq!(calculate_accountability_score(Some("Qualified Audit Opinion")), Some(dec!(50.0)));
        assert_eq!(calculate_accountability_score(Some("Adverse Audit Opinion")), Some(dec!(0.0)));
        assert_eq!(calculate_accountability_score(Some("Disclaimer Of Audit Opinion")), Some(dec!(0.0)));
        assert_eq!(calculate_accountability_score(Some("Unknown Outcome")), Some(dec!(0.0))); // Default
        assert_eq!(calculate_accountability_score(None), None);
    }

    #[test]
    fn test_clamping() {
        // Test case that would result in > 100 without clamping
         let input_high = ScoringInput {
             revenue: Some(dec!(100.0)),
             expenditure: Some(dec!(50.0)), // High efficiency
             capital_expenditure: Some(dec!(50.0)), // High infra
             debt: Some(dec!(0.0)), // Low debt
             audit_outcome: Some("Clean Audit".to_string()),
             population: Some(1.0), // Very high R/Capita
        };
         // Expected sub-scores (approx): Acc=100, Infra=100, Eff=100, FinHealth=100
         let score_high = calculate_financial_score(&input_high);
         assert_eq!(score_high.unwrap(), dec!(100.00));

         // Test case that would result in < 0 without clamping (via high debt/opex ratio)
         let input_low = ScoringInput {
             revenue: Some(dec!(100.0)),
             expenditure: Some(dec!(200.0)),
             capital_expenditure: Some(dec!(10.0)), // Low infra: 10/200=5 -> 5 score
             debt: Some(dec!(200.0)), // High debt ratio: 200/100=2 -> DebtScore=max(0, 1-2)*100 = 0
             audit_outcome: Some("Adverse Audit Opinion".to_string()), // Acc=0
             population: Some(1000.0), // Low R/Capita -> Score ~0
        };
         // FinHealth ~ 0
         // Efficiency: OpEx = 190. Ratio=190/100=1.9. Score = max(0, 1-1.9)*100 = 0
         // Total -> weighted sum of mostly zeros -> Should clamp to 0
         let score_low = calculate_financial_score(&input_low);
         assert_eq!(score_low.unwrap(), dec!(0.00));
    }

     #[test]
     fn test_zero_division_handling() {
         // Infra score with zero expenditure
         assert_eq!(calculate_infra_score(Some(dec!(100)), Some(dec!(0))), None);
         // Efficiency score with zero revenue
         assert_eq!(calculate_efficiency_score(Some(dec!(100)), Some(dec!(20)), Some(dec!(0))), None);
         // Debt subscore with zero revenue
         assert_eq!(calculate_debt_subscore(Some(dec!(100)), Some(dec!(0))), None);
         // RevPerCap subscore with zero population
         assert_eq!(calculate_rev_per_cap_subscore(Some(dec!(100)), Some(0.0)), None);

     }
}