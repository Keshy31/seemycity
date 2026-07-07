<script lang="ts">
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
</script>

<svelte:head>
  <title>{$page.status} | SeeMyCity</title>
</svelte:head>

<div class="error-page">
  <Icon icon="mdi:map-marker-question-outline" class="error-page-icon" width="64" height="64" />
  <h1>{$page.status}</h1>
  <p>{$page.error?.message ?? 'Something went wrong.'}</p>
  <a href="/" class="home-link">
    <Icon icon="mdi:map" />
    <span>Back to the map</span>
  </a>
</div>

<style lang="scss">
  .error-page {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    min-height: 60vh;
    gap: var(--spacing-md);
    text-align: center;
    padding: var(--spacing-xl);
    color: var(--text-muted-color);

    h1 {
      font-size: 3rem;
      margin: 0;
      color: var(--text-heading-color);
    }

    p {
      font-size: var(--font-size-lg);
      max-width: 32rem;
      margin: 0;
    }
  }

  .home-link {
    display: inline-flex;
    align-items: center;
    gap: var(--spacing-sm);
    margin-top: var(--spacing-lg);
    background-color: var(--primary-color);
    color: var(--text-inverse-color);
    padding: var(--spacing-sm) var(--spacing-lg);
    border-radius: var(--border-radius-md);
    text-decoration: none;
    font-weight: 600;
    transition: background-color 0.2s ease;

    &:hover {
      background-color: var(--accent-color-hover);
    }
  }
</style>
