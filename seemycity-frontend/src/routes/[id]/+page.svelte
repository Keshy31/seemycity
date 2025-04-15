<script lang="ts">
  import { page } from '$app/stores';
  import Icon from '@iconify/svelte';
  import { onMount } from 'svelte';

  interface MunicipalityDetails {
    id: string;
    name: string;
    province: string;
    population: number | null;
    classification: string | null;
    website: string | null;
    financial_year: number | null;
    revenue: number | null;
    expenditure: number | null;
    debt: number | null;
    audit_outcome: string | null;
  }

  $: id = $page.params.id;

  let muniDetails: MunicipalityDetails | null = null;
  let isLoading = true;
  let error: string | null = null;

  onMount(async () => {
    if (!id) {
      error = "Municipality ID not found in URL.";
      isLoading = false;
      return;
    }
    isLoading = true;
    error = null;
    try {
      const response = await fetch(`/api/municipalities/${id}`);
      if (!response.ok) {
        if (response.status === 404) {
          throw new Error(`Municipality with ID '${id}' not found.`);
        } else {
          throw new Error(`HTTP error! status: ${response.status}`);
        }
      }
      muniDetails = await response.json();
      console.log('Fetched Muni Details:', muniDetails);
    } catch (e: any) {
      console.error('Error fetching municipality details:', e);
      error = e.message || 'Failed to load municipality data.';
    } finally {
      isLoading = false;
    }
  });

  function formatCurrency(value: number | null | undefined): string {
    if (value === null || value === undefined) return 'N/A';
    if (Math.abs(value) >= 1e9) {
      return `R ${(value / 1e9).toFixed(1)} B`;
    } else if (Math.abs(value) >= 1e6) {
      return `R ${(value / 1e6).toFixed(1)} M`;
    } else {
      return `R ${value.toLocaleString()}`;
    }
  }
</script>

{#if isLoading}
  <h1>Loading Municipality Details...</h1>
  <p>Please wait.</p>
{:else if error}
  <h1>Error</h1>
  <p>{error}</p>
{:else if muniDetails}
  <h1>{muniDetails.name} Details</h1>
  <div class="flex justify-between mb-6">
    <div class="text-left">
      <p class="text-lg">Province: {muniDetails.province}</p>
      <p class="text-lg">Population: {muniDetails.population?.toLocaleString() ?? 'N/A'}</p>
      <p class="text-sm text-gray-600">Financial Year: {muniDetails.financial_year ?? 'N/A'}</p>
    </div>
    <div class="text-right">
      <p class="text-lg">(Score calculation pending)</p>
    </div>
  </div>

  <div class="mt-6 grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:cash-multiple" class="text-2xl mx-auto mb-2 text-teal-600" />
      <p class="font-semibold">Revenue</p>
      <p>{formatCurrency(muniDetails.revenue)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:bank-minus" class="text-2xl mx-auto mb-2 text-red-600" />
      <p class="font-semibold">Debt</p>
      <p>{formatCurrency(muniDetails.debt)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:home-city-outline" class="text-2xl mx-auto mb-2 text-blue-600" />
      <p class="font-semibold">Expenditure</p>
      <p>{formatCurrency(muniDetails.expenditure)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:scale-balance" class="text-2xl mx-auto mb-2 text-orange-600" />
      <p class="font-semibold">Audit</p>
      <p>{muniDetails.audit_outcome ?? 'N/A'}</p>
    </div>
  </div>

{:else if id}
  <h1>Municipality Not Found</h1>
  <p>Could not find details for municipality ID: <strong>{id}</strong></p>
{:else}
  <h1>Invalid Request</h1>
  <p>No Municipality ID specified.</p>
{/if}

<style>
  h1 {
    color: var(--primary-color, #008080);
    margin-bottom: 1rem;
  }
  strong {
    font-weight: bold;
  }
</style>