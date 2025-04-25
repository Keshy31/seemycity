<script lang="ts">
    import Icon from '@iconify/svelte';
</script>

<div class="spinner-container">
    <Icon icon="mdi:loading" class="spinner-icon" />
    <span>Loading...</span>
</div>

<style lang="scss">
    @import '../../../styles/variables.scss'; // Corrected path

    .spinner-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        color: var(--text-muted-color); // Corrected variable
        gap: 0.5rem;
    }

    .spinner-icon {
        font-size: 2.5rem;
        color: var(--accent-color); // Use CSS variable
        animation: spin 1.5s linear infinite;
    }

    @keyframes spin {
        from {
            transform: rotate(0deg);
        }
        to {
            transform: rotate(360deg);
        }
    }
</style>