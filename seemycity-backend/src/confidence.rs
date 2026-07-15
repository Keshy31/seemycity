//! Data-confidence evaluation for municipality-year financial figures.
//!
//! The Treasury data is self-reported by municipalities; the towns with the
//! worst governance also file the least plausible numbers (a municipality of
//! 100k+ people reporting R793k annual revenue, negative liabilities, …).
//! Presenting such artifacts as facts would overstate or understate their
//! health, so every row gets a confidence grade shown alongside its scores.
//!
//! Grades:
//! - `ok`         — figures pass all plausibility checks
//! - `suspect`    — at least one check raises doubt (shown with a caveat)
//! - `unreliable` — figures fail hard checks and should not be trusted
//!
//! A `None` grade means "not yet evaluated" (legacy rows); the healing pass
//! backfills those from stored values without refetching.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub const CONFIDENCE_OK: &str = "ok";
pub const CONFIDENCE_SUSPECT: &str = "suspect";
pub const CONFIDENCE_UNRELIABLE: &str = "unreliable";

/// Inputs for a confidence evaluation. All fields optional — checks that lack
/// inputs are skipped rather than failed.
#[derive(Debug, Default, Clone)]
pub struct ConfidenceInput {
    pub revenue: Option<Decimal>,
    pub operational_expenditure: Option<Decimal>,
    pub capital_expenditure: Option<Decimal>,
    pub debt: Option<Decimal>,
    pub population: Option<u32>,
    /// The Treasury cube's own total-revenue rollup (item 2900), when the row
    /// was just fetched. `|checksum - revenue|` should be ~0; a mismatch means
    /// our item summation and the cube disagree about this municipality.
    pub revenue_checksum: Option<Decimal>,
}

/// Result of evaluating one municipality-year.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfidenceGrade {
    pub grade: &'static str,
    /// Human-readable reasons, present unless the grade is `ok`.
    pub notes: Option<String>,
}

/// Evaluates plausibility of one municipality-year's raw figures.
pub fn evaluate(input: &ConfidenceInput) -> ConfidenceGrade {
    let mut suspect: Vec<String> = Vec::new();
    let mut unreliable: Vec<String> = Vec::new();

    // Hard check: negative values are accounting artifacts, not facts.
    for (name, value) in [
        ("revenue", input.revenue),
        ("operational expenditure", input.operational_expenditure),
        ("capital expenditure", input.capital_expenditure),
        ("debt", input.debt),
    ] {
        if let Some(v) = value {
            if v < Decimal::ZERO {
                unreliable.push(format!("negative {name} reported"));
            }
        }
    }

    // Hard check: revenue implausibly small for the population served.
    // A town of 20k+ people cannot run on under R10m of annual revenue —
    // this pattern indicates an incomplete return, not a poor municipality.
    if let (Some(rev), Some(pop)) = (input.revenue, input.population) {
        if pop >= 20_000 && rev >= Decimal::ZERO && rev < dec!(10_000_000) {
            unreliable.push(format!("revenue of R{rev:.0} is implausibly low for a population of {pop}"));
        }
    }

    // Ratio checks (both sides must be present and positive).
    if let (Some(opex), Some(rev)) = (input.operational_expenditure, input.revenue) {
        if rev > Decimal::ZERO && opex > Decimal::ZERO {
            let ratio = opex / rev;
            if ratio > dec!(3.0) {
                unreliable.push(format!(
                    "operating spend is {ratio:.1}x revenue — one side of the statement is likely missing"
                ));
            } else if ratio < dec!(0.1) {
                suspect.push(format!("operating spend is only {ratio:.2}x revenue"));
            }
        }
    }
    if let (Some(debt), Some(rev)) = (input.debt, input.revenue) {
        if rev > Decimal::ZERO && debt > Decimal::ZERO {
            let ratio = debt / rev;
            if ratio > dec!(5.0) {
                suspect.push(format!("reported liabilities are {ratio:.1}x annual revenue"));
            }
        }
    }

    // Incomplete statement: one side reported without the other.
    match (input.revenue, input.operational_expenditure) {
        (Some(_), None) => suspect.push("expenditure missing while revenue is reported".to_string()),
        (None, Some(_)) => suspect.push("revenue missing while expenditure is reported".to_string()),
        _ => {}
    }

    // Checksum: the cube's own revenue rollup should match our summation.
    if let (Some(rev), Some(check)) = (input.revenue, input.revenue_checksum) {
        let denom = rev.abs().max(Decimal::ONE);
        let deviation = (check - rev).abs() / denom;
        if deviation > dec!(0.01) {
            suspect.push(format!(
                "revenue checksum deviates by {:.1}% from the Treasury rollup",
                deviation * dec!(100)
            ));
        }
    }

    if !unreliable.is_empty() {
        unreliable.extend(suspect);
        ConfidenceGrade { grade: CONFIDENCE_UNRELIABLE, notes: Some(unreliable.join("; ")) }
    } else if !suspect.is_empty() {
        ConfidenceGrade { grade: CONFIDENCE_SUSPECT, notes: Some(suspect.join("; ")) }
    } else {
        ConfidenceGrade { grade: CONFIDENCE_OK, notes: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_metro_figures_are_ok() {
        // Cape Town FY2024 shape
        let g = evaluate(&ConfidenceInput {
            revenue: Some(dec!(61_841_000_000)),
            operational_expenditure: Some(dec!(58_668_000_000)),
            capital_expenditure: Some(dec!(14_278_000_000)),
            debt: Some(dec!(25_538_000_000)),
            population: Some(4_772_846),
            revenue_checksum: Some(dec!(61_841_000_000)),
        });
        assert_eq!(g.grade, CONFIDENCE_OK);
        assert!(g.notes.is_none());
    }

    #[test]
    fn ratlou_style_figures_are_unreliable() {
        // Real FY2024 pattern: R793k revenue for 100k+ people, negative debt,
        // opex 16.9x revenue — a Disclaimer-audit municipality's filing.
        let g = evaluate(&ConfidenceInput {
            revenue: Some(dec!(793_551)),
            operational_expenditure: Some(dec!(13_380_993)),
            capital_expenditure: Some(dec!(7_122_082)),
            debt: Some(dec!(-1_737_690)),
            population: Some(100_000),
            revenue_checksum: None,
        });
        assert_eq!(g.grade, CONFIDENCE_UNRELIABLE);
        let notes = g.notes.unwrap();
        assert!(notes.contains("negative debt"));
        assert!(notes.contains("implausibly low"));
        assert!(notes.contains("x revenue"));
    }

    #[test]
    fn gamagara_style_debt_is_suspect() {
        // R13.3bn liabilities against R673m revenue (19.8x)
        let g = evaluate(&ConfidenceInput {
            revenue: Some(dec!(673_431_654)),
            operational_expenditure: Some(dec!(654_605_099)),
            capital_expenditure: Some(dec!(155_193_495)),
            debt: Some(dec!(13_301_932_796)),
            population: Some(53_000),
            revenue_checksum: None,
        });
        assert_eq!(g.grade, CONFIDENCE_SUSPECT);
        assert!(g.notes.unwrap().contains("liabilities"));
    }

    #[test]
    fn checksum_mismatch_is_suspect() {
        let g = evaluate(&ConfidenceInput {
            revenue: Some(dec!(1_000_000)),
            operational_expenditure: Some(dec!(900_000)),
            revenue_checksum: Some(dec!(1_100_000)), // 10% off
            ..Default::default()
        });
        assert_eq!(g.grade, CONFIDENCE_SUSPECT);
        assert!(g.notes.unwrap().contains("checksum"));
    }

    #[test]
    fn one_sided_statement_is_suspect() {
        let g = evaluate(&ConfidenceInput {
            revenue: Some(dec!(5_000_000_000)),
            population: Some(1_000_000),
            ..Default::default()
        });
        assert_eq!(g.grade, CONFIDENCE_SUSPECT);
        assert!(g.notes.unwrap().contains("expenditure missing"));
    }

    #[test]
    fn empty_input_is_ok_not_penalised() {
        // Absence of data is handled by NULL scores, not by confidence.
        let g = evaluate(&ConfidenceInput::default());
        assert_eq!(g.grade, CONFIDENCE_OK);
    }
}
