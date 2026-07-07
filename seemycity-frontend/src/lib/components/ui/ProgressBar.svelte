<script lang="ts">
	import { getScoreColorVarName } from '$lib/utils/formatUtils';

	/** The current value (typically 0-100) */
	export let value: number | null | undefined = 0;
	/** The maximum value for the scale (usually 100) */
	export let max: number = 100;
	/** Height of the progress bar */
	export let height: string = '10px'; // Slightly taller for better visibility
	/** Optional specific background color CSS variable name */
	export let backgroundColorVar: string = '--background-offset-dark'; // Default background
	/** Optional border radius */
	export let borderRadius: string = 'var(--border-radius-md)';

	$: displayValue = value ?? 0;
	$: percentage = max > 0 ? Math.max(0, Math.min(100, (displayValue / max) * 100)) : 0;

	// Directly get the CSS variable name for the score color
	$: scoreColorVar = getScoreColorVarName(displayValue);
</script>

<div
	class="progress-bar-container"
	style="--progress-height: {height}; --progress-bg: var({backgroundColorVar}); --progress-radius: {borderRadius};"
	role="progressbar"
	aria-valuenow={displayValue}
	aria-valuemin="0"
	aria-valuemax={max}
	title={`Score: ${displayValue.toFixed(1)} / ${max}`}
>
	<div
		class="progress-bar-value"
		style="width: {percentage}%; background-color: var({scoreColorVar});"
	></div>
</div>

<style lang="scss">
	@use '../../../styles/variables' as *;

	.progress-bar-container {
		width: 100%;
		height: var(--progress-height);
		background-color: var(--progress-bg, var(--background-offset-dark));
		border-radius: var(--progress-radius);
		overflow: hidden;
		box-shadow: inset 0 1px 3px rgba(0, 0, 0, 0.15); // Deeper inner shadow for depth
	}

	.progress-bar-value {
		height: 100%;
		border-radius: var(--progress-radius);
		transition: width 0.4s cubic-bezier(0.25, 1, 0.5, 1); // Smoother, more satisfying transition

		// Add a subtle gradient for a more polished look
		background-image: linear-gradient(
			180deg,
			rgba(255, 255, 255, 0.15) 0%,
			rgba(255, 255, 255, 0) 100%
		);
	}
</style>
