/**
 * Formats a numeric value as South African Rand (ZAR) currency.
 * Displays whole Rands by default.
 * @param value - The numeric value to format.
 * @returns A formatted currency string (e.g., "R 1,234") or "N/A".
 */
export function formatCurrency(value: number | null | undefined): string {
	if (value === null || value === undefined) {
		return 'N/A';
	}

	// Use Intl.NumberFormat for locale-aware currency formatting
	return new Intl.NumberFormat('en-ZA', {
		style: 'currency',
		currency: 'ZAR',
		minimumFractionDigits: 0, // No cents
		maximumFractionDigits: 0 // No cents
	}).format(value);
}

/**
 * Calculates and formats a percentage value from a numerator and denominator.
 * Handles null/undefined inputs and division by zero.
 * @param numerator - The numerator value.
 * @param denominator - The denominator value.
 * @returns A formatted percentage string (e.g., "12.3%") or "N/A".
 */
export function formatPercentage(
	numerator: number | null | undefined,
	denominator: number | null | undefined
): string {
	if (
		numerator === null ||
		numerator === undefined ||
		denominator === null ||
		denominator === undefined ||
		denominator === 0 // Avoid division by zero
	) {
		return 'N/A';
	}

	const percentage = (numerator / denominator) * 100;
	// Format to one decimal place
	return percentage.toFixed(1) + '%';
}

/**
 * Formats a website URL for cleaner display.
 * Removes 'http(s)://' and 'www.' prefixes.
 * @param url - The website URL string.
 * @returns A cleaned URL string or "N/A".
 */
export function formatWebsite(url: string | null | undefined): string {
	if (!url) return 'N/A';
	// Remove http(s):// and www. for cleaner display
	return url.replace(/^(https?:\/\/)?(www\.)?/, '').replace(/\/$/, ''); // Also remove trailing slash
}

/**
 * Formats a population number with locale-specific separators.
 * @param pop - The population number.
 * @returns A formatted population string (e.g., "1,234,567") or "N/A".
 */
export function formatPopulation(pop: number | null | undefined): string {
	if (pop === null || pop === undefined) return 'N/A';
	// Use Intl.NumberFormat for locale-aware number formatting without decimals
	return new Intl.NumberFormat('en-ZA', {
		maximumFractionDigits: 0
	}).format(pop);
}

/**
 * Formats a score (0-100) to one decimal place.
 * Clamps the score between 0 and 100.
 * @param score - The score number.
 * @returns A formatted score string (e.g., "75.3") or "N/A".
 */
export function formatScore(score: number | null | undefined): string {
	if (score == null) return 'N/A';
	// Clamp score between 0 and 100 before formatting
	const clampedScore = Math.max(0, Math.min(100, score));
	return clampedScore.toFixed(1); // Use one decimal place
}

/**
 * Returns the CSS variable name for score color based on value.
 * This is the preferred utility for components that need just the variable.
 * @param score - The score number.
 * @returns A CSS variable name (e.g., "--score-color-high").
 */
export function getScoreColorVarName(score: number | null | undefined): string {
	if (score == null) return '--text-muted-color';
	if (score >= 70) return '--score-high-color';
	if (score >= 40) return '--score-medium-color';
	return '--score-low-color';
}

/**
 * Returns an inline CSS style string for score color based on value.
 * Uses CSS variables defined in _variables.scss.
 * @param score - The score number.
 * @returns A CSS style string (e.g., "color: var(--score-high-color);").
 */
export function getScoreColorStyle(score: number | null | undefined): string {
	// Now uses the centralized logic from getScoreColorVarName
	return `color: var(${getScoreColorVarName(score)});`;
}

/**
 * Returns an Iconify icon name reflecting the score band. Color the icon at the
 * call site (e.g. via getScoreColorStyle) — the icon itself is monochrome.
 * @param score - The score number.
 * @returns An Iconify icon name (e.g., "mdi:emoticon-happy-outline").
 */
export function getScoreStatusIcon(score: number | null | undefined): string {
	if (score == null) return 'mdi:help-circle-outline'; // No data
	if (score >= 70) return 'mdi:emoticon-happy-outline';
	if (score >= 40) return 'mdi:emoticon-neutral-outline';
	return 'mdi:emoticon-sad-outline';
}

// Add other formatting functions here as needed (e.g., dates)
