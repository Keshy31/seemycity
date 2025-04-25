<script lang="ts">
    import Icon from '@iconify/svelte';

    export let message: string = 'An error occurred while loading data.'; // Default message
    export let errorDetails: string | undefined = undefined; // Optional more detailed error
</script>

<div class="error-container">
    <Icon icon="mdi:alert-circle-outline" class="error-icon" />
    <h4>{message}</h4>
    {#if errorDetails}
        <p class="error-details">{errorDetails}</p>
    {/if}
    <!-- Optionally add a retry button -->
    <!-- <button on:click={() => dispatch('retry')}>Retry</button> -->
</div>

<style lang="scss">
    @import '../../../styles/variables'; // Consistent import

    .error-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        text-align: center;
        color: var(--text-muted-color);
        background-color: var(--error-color-bg); // Use CSS var
        border: 1px solid var(--error-color-light); // Use CSS var
        border-radius: var(--border-radius-medium); // Use CSS var
        gap: 0.75rem;
    }

    .error-icon {
        font-size: 2.5rem;
        color: var(--error-color); // Already CSS var
    }

    h4 {
        margin: 0;
        color: var(--error-color-dark); // Use CSS var
        font-size: 1.1rem;
    }

    .error-details {
        font-size: 0.9rem;
        color: var(--text-muted-color); // Already CSS var
        max-width: 400px; // Prevent overly wide details
    }
</style>