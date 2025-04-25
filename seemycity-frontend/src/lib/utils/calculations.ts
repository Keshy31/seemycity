/**
 * Calculates the Capital Expenditure (CapEx) Ratio.
 * CapEx Ratio = Capital Expenditure / (Capital Expenditure + Operational Expenditure)
 * Represents the proportion of total expenditure dedicated to capital projects.
 * Handles null/undefined inputs and division by zero.
 */
export function calculateCapexRatio(
	capitalExpenditure: number | null | undefined,
	opex: number | null | undefined
): number | null {
	const cap = capitalExpenditure ?? 0;
	const op = opex ?? 0;
	const totalExpenditure = cap + op;

	if (totalExpenditure === 0) {
		return 0; // Or null if 0 expenditure means the ratio is undefined
	}
	if (cap === 0) {
		return 0;
	}

	return cap / totalExpenditure;
}

/**
 * Calculates the Operational Expenditure (OpEx) Ratio.
 * OpEx Ratio = Operational Expenditure / Total Revenue
 * Represents the proportion of revenue consumed by operational costs.
 * Handles null/undefined inputs and division by zero.
 */
export function calculateOpexRatio(
	opex: number | null | undefined,
	revenue: number | null | undefined
): number | null {
	const op = opex ?? 0;
	const rev = revenue ?? 0;

	if (rev === 0) {
		// If revenue is zero, the concept of OpEx ratio might be undefined or infinite.
		// Returning null might be safest, or 0 if OpEx is also 0.
		return op === 0 ? 0 : null;
	}
	if (op === 0) {
		return 0;
	}

	return op / rev;
}

/**
 * Calculates the Debt Ratio.
 * Debt Ratio = Total Debt / Total Revenue
 * Represents the municipality's debt level relative to its revenue.
 * Handles null/undefined inputs and division by zero.
 */
export function calculateDebtRatio(
	debt: number | null | undefined,
	revenue: number | null | undefined
): number | null {
	const d = debt ?? 0;
	const rev = revenue ?? 0;

	if (rev === 0) {
		// Similar to OpEx ratio, if revenue is zero, debt ratio could be seen as infinite or undefined.
		return d === 0 ? 0 : null;
	}
	if (d === 0) {
		return 0;
	}

	return d / rev;
}

/**
 * Calculates Revenue Per Capita.
 * Revenue Per Capita = Total Revenue / Population
 * Represents the average revenue generated per resident.
 * Handles null/undefined inputs and division by zero (zero population).
 */
export function calculateRevenuePerCapita(
	revenue: number | null | undefined,
	population: number | null | undefined
): number | null {
	const rev = revenue ?? 0;
	const pop = population ?? 0;

	if (pop === 0) {
		// Revenue per capita is undefined if population is zero.
		return null; // Or return 0 if revenue is also 0?
	}
	if (rev === 0) {
		return 0;
	}

	return rev / pop;
}