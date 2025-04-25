<script lang="ts">
    import type { MunicipalityDetail } from '$lib/types'; // Use the correct type
    import ComparisonCard from './ComparisonCard.svelte'; // The card component we'll create next

    // Accept the array of municipalities to compare
    export let municipalities: MunicipalityDetail[];
</script>

<div class="comparison-container">
    {#if municipalities && municipalities.length > 0}
        <h1 class="page-title">Municipality Comparison</h1>
        <div class="cards-wrapper">
            {#each municipalities as muni (muni.id)}
                <ComparisonCard municipality={muni} />
            {/each}
        </div>
    {:else}
        <!-- Optional: Message if array is somehow empty -->
        <p>No municipalities selected for comparison.</p>
    {/if}
</div>

<style lang="scss">
    @use '../../../styles/variables' as *; // Import variables

    .comparison-container {
        padding: var(--spacing-lg, 2rem);
        max-width: 100%;
        margin: 0 auto; // Center container if needed
    }

    .page-title {
        font-size: var(--font-size-h2, 1.875rem);
        font-weight: var(--font-weight-bold, 700);
        margin-bottom: var(--spacing-lg, 1.5rem);
        font-family: var(--font-family-headings, 'Ubuntu', sans-serif);
        text-align: center; // Center title
        color: var(--text-primary);
    }

    .cards-wrapper {
        display: flex;
        flex-direction: row; // Arrange cards horizontally
        gap: var(--spacing-lg, 1.5rem); // Space between cards
        overflow-x: auto; // Allow horizontal scrolling if cards overflow
        padding-bottom: var(--spacing-md, 1rem); // Add some padding for the scrollbar
        justify-content: center; // Center cards if they don't fill the width

        // Optional: Add snapping behavior for scrolling
        scroll-snap-type: x mandatory;
        & > * {
            scroll-snap-align: center;
        }

        // Hide scrollbar visually but keep functionality
        &::-webkit-scrollbar {
            display: none; // For Chrome, Safari, Opera
        }
        -ms-overflow-style: none; // For IE and Edge
        scrollbar-width: none; // For Firefox
    }
</style>