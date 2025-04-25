<script lang="ts">
    import { getScoreColorStyle } from '$lib/utils/formatUtils';

    /** The current value (typically 0-100) */
    export let value: number | null | undefined = 0;
    /** The maximum value for the scale (usually 100) */
    export let max: number = 100;
    /** Height of the progress bar */
    export let height: string = '8px';
    /** Optional specific background color CSS variable name */
    export let backgroundColorVar: string = '--background-offset-dark'; // Default background
    /** Optional border radius */
    export let borderRadius: string = '4px';

    $: displayValue = value ?? 0; // Use 0 if value is null/undefined
    $: percentage = max > 0 ? Math.max(0, Math.min(100, (displayValue / max) * 100)) : 0;

    // Get the color style string (e.g., 'color: var(--score-color-high);')
    // We need to extract the variable name (e.g., '--score-color-high')
    $: scoreColorStyleString = getScoreColorStyle(displayValue);
    $: scoreColorVar = scoreColorStyleString.match(/var\((.*?)\)/)?.[1] || '--text-muted-color'; // Extract variable or default

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
    .progress-bar-container {
        width: 100%;
        height: var(--progress-height);
        background-color: var(--progress-bg);
        border-radius: var(--progress-radius);
        overflow: hidden;
        box-shadow: inset 0 1px 2px rgba(0, 0, 0, 0.1); // Subtle inner shadow
    }

    .progress-bar-value {
        height: 100%;
        border-radius: var(--progress-radius);
        transition: width 0.3s ease-in-out, background-color 0.3s ease-in-out; // Smooth transitions
        box-shadow: inset 0 -1px 1px rgba(255, 255, 255, 0.1); // Subtle inner highlight
    }
</style>