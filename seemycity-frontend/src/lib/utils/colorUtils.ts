/**
 * Determines the CSS color style based on a financial health score.
 * Uses HSL color space for smooth interpolation between red, orange, and green.
 * Scores map as: 0 -> Red (0deg), 50 -> Orange (60deg), 100 -> Green (120deg).
 * @param score - The financial health score (0-100).
 * @returns A CSS style string (e.g., 'color: hsl(120, 70%, 45%);').
 */
export function getScoreColorStyle(score: number): string {
  // Clamp score between 0 and 100
  const clampedScore = Math.max(0, Math.min(100, score));

  // Interpolate hue: 0 maps to 0 (red), 50 maps to 60 (orange/yellow), 100 maps to 120 (green)
  const hue = (clampedScore / 100) * 120;

  // Keep saturation and lightness constant for vibrant colors
  const saturation = '70%'; // Adjust saturation as needed
  const lightness = '45%'; // Adjust lightness as needed

  return `color: hsl(${hue.toFixed(0)}, ${saturation}, ${lightness});`;
}

/**
 * Determines the appropriate Iconify icon name based on a score range.
 * @param score - The score (0-100).
 * @returns An Iconify icon name string (e.g., 'mdi:emoticon-happy-outline').
 */
export function getScoreStatusIcon(score: number): string {
  if (score >= 75) {
    return 'mdi:emoticon-happy-outline'; // Good
  } else if (score >= 40) {
    return 'mdi:emoticon-neutral-outline'; // Okay
  } else {
    return 'mdi:emoticon-sad-outline'; // Poor
  }
}