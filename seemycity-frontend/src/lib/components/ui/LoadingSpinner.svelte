<script lang="ts">
    import Icon from '@iconify/svelte';
</script>

<div class="spinner-container" role="status" aria-live="polite">
    <Icon icon="mdi:loading" class="spinner-icon" />
    <span class="spinner-text">Loading...</span>
</div>

<style lang="scss">
    @use '../../../styles/variables' as *;

    .spinner-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: var(--spacing-xl);
        gap: var(--spacing-md);
    }

    .spinner-text {
        color: var(--text-muted-color);
        font-size: var(--font-size-sm);
        font-weight: var(--font-weight-medium);
        letter-spacing: 0.05em;
    }

    .spinner-icon {
        font-size: 2.5rem;
        color: var(--primary-color);
        animation: pulse-and-spin 1.5s ease-in-out infinite;
    }

    @keyframes pulse-and-spin {
        0% {
            transform: rotate(0deg) scale(1);
        }
        50% {
            transform: rotate(180deg) scale(0.9);
        }
        100% {
            transform: rotate(360deg) scale(1);
        }
    }
</style>