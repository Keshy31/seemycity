/**
 * Maps an audit outcome string to an appropriate Iconify icon name.
 * Handles various string variations and provides a default icon.
 * @param outcome - The audit outcome string (e.g., "Unqualified opinion with no findings").
 * @returns An Iconify icon string (e.g., 'mdi:check-circle-outline').
 */
export function getAuditIcon(outcome: string | null | undefined): string {
  if (!outcome) return 'mdi:help-circle-outline'; // Default for null/undefined

  const lowerOutcome = outcome.toLowerCase().trim();

  switch (lowerOutcome) {
    case 'unqualified - no findings':
    case 'unqualified opinion with no findings':
      return 'mdi:check-circle-outline'; // Clean
    case 'unqualified - emphasis of matter items':
    case 'financially unqualified opinion': // Common alias
    case 'unqualified opinion with findings':
      return 'mdi:information-outline'; // Clean but with notes
    case 'qualified':
    case 'qualified opinion':
      return 'mdi:alert-circle-outline'; // Issues found
    case 'adverse':
    case 'adverse opinion':
      return 'mdi:close-circle-outline'; // Major issues
    case 'disclaimer':
    case 'disclaimer of opinion':
    case 'disclaimer with findings':
      return 'mdi:comment-question-outline'; // Unable to audit
    case 'outstanding': // Financial statements not submitted
    case 'financial statements not submitted':
      return 'mdi:clock-alert-outline'; // Not submitted
    default:
      console.warn('Unknown audit outcome in getAuditIcon:', outcome);
      return 'mdi:help-circle-outline'; // Default for unknown
  }
}

/**
 * Maps an audit outcome string to a CSS color style variable.
 * Uses predefined CSS variables for consistency.
 * @param outcome - The audit outcome string.
 * @returns A CSS style string (e.g., 'color: var(--audit-clean-color);').
 */
export function getAuditOutcomeColorStyle(outcome: string | null | undefined): string {
  if (!outcome) return 'color: var(--text-muted-color);'; // Use muted color for null/undefined

  const lowerOutcome = outcome.toLowerCase().trim();

  switch (lowerOutcome) {
    case 'unqualified - no findings':
    case 'unqualified opinion with no findings':
      return 'color: var(--audit-clean-color);'; // Defined in _variables.scss
    case 'unqualified - emphasis of matter items':
    case 'financially unqualified opinion':
    case 'unqualified opinion with findings':
      return 'color: var(--audit-emphasis-color);'; // Defined in _variables.scss
    case 'qualified':
    case 'qualified opinion':
      return 'color: var(--audit-qualified-color);'; // Defined in _variables.scss
    case 'adverse':
    case 'adverse opinion':
    case 'disclaimer':
    case 'disclaimer of opinion':
    case 'disclaimer with findings':
      return 'color: var(--audit-adverse-disclaimer-color);'; // Defined in _variables.scss
    case 'outstanding':
    case 'financial statements not submitted':
      return 'color: var(--audit-outstanding-color);'; // Defined in _variables.scss
    default:
      console.warn('Unknown audit outcome in getAuditOutcomeColorStyle:', outcome);
      return 'color: var(--text-muted-color);'; // Default muted color
  }
}

/**
 * Maps an audit outcome string to a user-friendly text representation.
 * Provides clearer labels for different outcomes.
 * @param outcome - The audit outcome string.
 * @returns A formatted string (e.g., 'Clean (Unqualified - No Findings)').
 */
export function getAuditOutcomeText(outcome: string | null | undefined): string {
  if (!outcome) return 'N/A';

  const lowerOutcome = outcome.toLowerCase().trim();

  switch (lowerOutcome) {
    case 'unqualified - no findings':
    case 'unqualified opinion with no findings':
      return 'Clean (Unqualified - No Findings)';
    case 'unqualified - emphasis of matter items':
    case 'unqualified opinion with findings': // Grouping similar concepts
      return 'Unqualified (Emphasis of Matter)';
    case 'financially unqualified opinion': // This seems less common, map to Emphasis for now?
       return 'Unqualified (Emphasis of Matter)'; // Or map specifically if needed
    case 'qualified':
    case 'qualified opinion':
      return 'Qualified Opinion';
    case 'adverse':
    case 'adverse opinion':
      return 'Adverse Opinion';
    case 'disclaimer':
    case 'disclaimer of opinion':
    case 'disclaimer with findings':
      return 'Disclaimer of Opinion';
    case 'outstanding':
    case 'financial statements not submitted':
      return 'Outstanding (Not Submitted)';
    default:
      console.warn('Unknown audit outcome in getAuditOutcomeText:', outcome);
      // Return the original outcome if unknown, but cleaned up slightly
      return outcome.length > 50 ? outcome.substring(0, 47) + '...' : outcome;
  }
}