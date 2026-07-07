<script lang="ts">
  import type { PageData } from './$types';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  import ComparisonContainer from '$lib/components/compare/ComparisonContainer.svelte';
  import Icon from '@iconify/svelte';

  export let data: PageData; // Resolved by the load function before render
</script>

<svelte:head>
  <title>Compare Municipalities | SeeMyCity</title>
  <meta name="description" content="Compare financial health details for selected municipalities." />
</svelte:head>

{#if data.municipalities && data.municipalities.length > 0}
  <ComparisonContainer municipalities={data.municipalities} />
{:else if data.error}
  <ErrorMessage message={`Error loading comparison data: ${data.error}`} />
{:else}
  <div class="no-data-container">
    <Icon icon="mdi:alert-circle-outline" />
    <h2>No Comparison Data Found</h2>
    <p>We couldn't find any data for the requested municipalities.</p>
    {#if data.requestedIds && data.requestedIds.length > 0}
      <p class="requested-ids">
        You requested: <strong>{data.requestedIds.join(', ')}</strong>
      </p>
    {/if}
    <a href="/" class="button-link">Back to Map</a>
  </div>
{/if}

<style lang="scss">
  @use '../../../styles/variables' as *;

  .no-data-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: var(--spacing-xxl) var(--spacing-lg);
    border: 2px dashed var(--border-color);
    border-radius: var(--border-radius-lg);
    background-color: var(--background-offset-color);
    color: var(--text-muted-color);
    min-height: 400px;
    margin: var(--spacing-lg);

    :global(.iconify) {
      font-size: 4rem;
      margin-bottom: var(--spacing-lg);
      color: var(--warning-color);
    }

    h2 {
      font-size: var(--font-size-xl);
      color: var(--text-heading-color);
      margin: 0 0 var(--spacing-sm) 0;
    }

    p {
      max-width: 450px;
      margin: 0;
      line-height: 1.6;
    }

    .requested-ids {
      margin-top: var(--spacing-md);
      font-size: var(--font-size-sm);
      color: var(--text-muted-color);

      strong {
        color: var(--text-color);
      }
    }

    .button-link {
      margin-top: var(--spacing-xl);
      padding: var(--spacing-sm) var(--spacing-lg);
      background-color: var(--primary-color);
      color: var(--button-text-color);
      text-decoration: none;
      border-radius: var(--border-radius-md);
      font-weight: 600;
      transition: background-color 0.2s ease;

      &:hover {
        background-color: var(--primary-color-dark);
      }
    }
  }
</style>