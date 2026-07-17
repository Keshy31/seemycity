// The 5 quantized score bands — the single source of truth shared by the map
// ramp, the legend, hover tooltips, and the detail page. Band colors live as
// CSS custom properties in _variables.scss (CVD-validated as a set).

export interface ScoreBand {
	/** Inclusive lower bound; a score belongs to the highest band whose min it clears. */
	min: number;
	/** Upper bound, display only (e.g. "60–79"). */
	max: number;
	label: string;
	/** One-line plain-English verdict for tooltips and cards. */
	verdict: string;
	cssVar: string;
}

/** Ordered best → worst, the order a legend reads in. */
export const SCORE_BANDS: ScoreBand[] = [
	{
		min: 80,
		max: 100,
		label: 'Thriving',
		verdict: 'Self-funded, maintained, and clean books.',
		cssVar: '--band-thriving-color'
	},
	{
		min: 60,
		max: 79,
		label: 'Healthy',
		verdict: 'Strong fundamentals with gaps to close.',
		cssVar: '--band-healthy-color'
	},
	{
		min: 40,
		max: 59,
		label: 'Strained',
		verdict: 'Getting by, but the cracks are showing.',
		cssVar: '--band-strained-color'
	},
	{
		min: 20,
		max: 39,
		label: 'In trouble',
		verdict: 'Struggling to fund, maintain, or account for itself.',
		cssVar: '--band-trouble-color'
	},
	{
		min: 0,
		max: 19,
		label: 'Critical',
		verdict: 'Failing on most measures that matter.',
		cssVar: '--band-critical-color'
	}
];

/** The sixth "band": no score at all. Missing or unreliable figures — never a judgement. */
export const NO_DATA_BAND = {
	label: 'No usable data',
	verdict: 'Figures missing or too unreliable to score.',
	cssVar: '--score-none-color'
} as const;

export function bandForScore(score: number | null | undefined): ScoreBand | null {
	if (score == null || Number.isNaN(score)) return null;
	return SCORE_BANDS.find((b) => score >= b.min) ?? SCORE_BANDS[SCORE_BANDS.length - 1];
}
