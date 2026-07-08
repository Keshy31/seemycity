<script lang="ts">
	import Icon from '@iconify/svelte';
	import type { DataConfidence } from '$lib/types';

	export let confidence: DataConfidence | null | undefined = undefined;
	export let notes: string | null | undefined = undefined;

	// Clean data needs no badge — only doubt is worth announcing.
	$: visible = confidence === 'suspect' || confidence === 'unreliable';
	$: label = confidence === 'unreliable' ? 'Figures unreliable' : 'Some figures look implausible';
	$: detail =
		confidence === 'unreliable'
			? 'The reported numbers fail basic plausibility checks — usually a sign of the poor bookkeeping its audit outcome describes. Scores for this year should not be taken at face value.'
			: 'One or more reported figures look unusual for a municipality of this size.';
</script>

{#if visible}
	<aside class="confidence-badge {confidence}" role="note">
		<Icon
			icon={confidence === 'unreliable' ? 'mdi:alert-octagon-outline' : 'mdi:alert-outline'}
			class="confidence-icon"
		/>
		<div class="confidence-text">
			<strong>{label}</strong>
			<p>{detail}</p>
			{#if notes}
				<p class="confidence-notes">Checks flagged: {notes}.</p>
			{/if}
		</div>
	</aside>
{/if}

<style lang="scss">
	.confidence-badge {
		display: flex;
		gap: var(--spacing-md);
		align-items: flex-start;
		padding: var(--spacing-md) var(--spacing-lg);
		border-radius: var(--border-radius-md);
		margin-bottom: var(--spacing-lg);
		font-size: var(--font-size-sm);
		border: 1px solid;

		&.suspect {
			background-color: color-mix(in srgb, var(--warning-color) 8%, var(--surface-color));
			border-color: color-mix(in srgb, var(--warning-color) 35%, transparent);
		}

		&.unreliable {
			background-color: color-mix(in srgb, var(--error-color) 7%, var(--surface-color));
			border-color: color-mix(in srgb, var(--error-color) 35%, transparent);
		}
	}

	// :global() because the Icon component's <svg> is outside Svelte's scoping.
	.confidence-badge :global(.confidence-icon) {
		font-size: 1.4rem;
		flex-shrink: 0;
		margin-top: 2px;
	}

	.confidence-badge.suspect :global(.confidence-icon) {
		color: var(--warning-color);
	}

	.confidence-badge.unreliable :global(.confidence-icon) {
		color: var(--error-color);
	}

	.confidence-text {
		strong {
			display: block;
			color: var(--text-heading-color);
			margin-bottom: var(--spacing-xs);
		}

		p {
			margin: 0;
			color: var(--text-muted-color);
			line-height: 1.5;
			max-width: 60ch;
		}

		.confidence-notes {
			margin-top: var(--spacing-xs);
			font-size: var(--font-size-xs);
		}
	}
</style>
