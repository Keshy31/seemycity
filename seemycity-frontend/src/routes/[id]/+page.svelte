<script lang="ts">
  import { page } from '$app/stores'; 
  import { dummyMunicipalityDetails } from '$lib/data/dummyStore'; 
  import type { MunicipalityDetails } from '$lib/data/dummyStore'; 
  import Icon from '@iconify/svelte'; 
 
  $: id = $page.params.id; 
 
  let currentMuni: MunicipalityDetails | null = null; 
  $: currentMuni = dummyMunicipalityDetails[id]; 
 
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

{#if currentMuni}
  <h1>{currentMuni.name} Details</h1>
  <div class="flex justify-between mb-6">
    <div class="text-left">
      <p class="text-lg">Province: {currentMuni.province}</p>
      <p class="text-lg">Population: {currentMuni.population?.toLocaleString() ?? 'N/A'}</p>
      <p class="text-sm text-gray-600">Financial Year: {currentMuni.year}</p>
    </div>
    <div class="text-right">
      <h2 class="text-4xl font-bold">Score: {currentMuni.score ?? 'N/A'} / 100</h2>
    </div>
  </div>

  <!-- Key Metrics Section -->
  <div class="mt-6 grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:cash-multiple" class="text-2xl mx-auto mb-2 text-teal-600" />
      <p class="font-semibold">Revenue</p>
      <p>{formatCurrency(currentMuni.revenue)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:bank-minus" class="text-2xl mx-auto mb-2 text-red-600" />
      <p class="font-semibold">Debt</p>
      <p>{formatCurrency(currentMuni.debt)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:home-city-outline" class="text-2xl mx-auto mb-2 text-blue-600" />
      <p class="font-semibold">Capex</p>
      <p>{formatCurrency(currentMuni.capital_expenditure)}</p>
    </div>
    <div class="p-4 bg-gray-100 rounded-lg shadow">
      <Icon icon="mdi:scale-balance" class="text-2xl mx-auto mb-2 text-orange-600" />
      <p class="font-semibold">Audit</p>
      <p>{currentMuni.audit_outcome}</p>
    </div>
  </div>

  <!-- Score Breakdown -->
  <div class="mt-8">
    <h3 class="text-2xl font-semibold mb-4">What’s behind this score?</h3>
    <div class="space-y-3">
      <div class="flex justify-between p-3 bg-green-50 rounded-md shadow-sm">
        <span><Icon icon="mdi:finance" class="inline mr-2"/>Financial Health (30%)</span>
        <span class="font-medium">{currentMuni.score_breakdown?.financial_health.toFixed(1)} pts</span>
      </div>
      <div class="flex justify-between p-3 bg-blue-50 rounded-md shadow-sm">
        <span><Icon icon="mdi:domain" class="inline mr-2"/>Infrastructure Investment (25%)</span>
        <span class="font-medium">{currentMuni.score_breakdown?.infrastructure.toFixed(1)} pts</span>
      </div>
      <div class="flex justify-between p-3 bg-orange-50 rounded-md shadow-sm">
        <span><Icon icon="mdi:cogs" class="inline mr-2"/>Efficiency & Service Delivery (25%)</span>
        <span class="font-medium">{currentMuni.score_breakdown?.efficiency.toFixed(1)} pts</span>
      </div>
      <div class="flex justify-between p-3 bg-purple-50 rounded-md shadow-sm">
        <span><Icon icon="mdi:check-decagram-outline" class="inline mr-2"/>Accountability (20%)</span>
        <span class="font-medium">{currentMuni.score_breakdown?.accountability.toFixed(1)} pts</span>
      </div>
    </div>
  </div>

{:else if id}
  <h1>Municipality Not Found</h1>
  <p>Could not find details for municipality ID: <strong>{id}</strong></p>
{:else}
  <h1>Loading Municipality Details...</h1>
  <p>Please wait.</p> 
{/if}

<style>
  /* Optional: Add some basic styling */
  h1 {
    /* Use --primary-color from ui settings or fallback */
    color: var(--primary-color, #008080); 
    margin-bottom: 1rem;
  }
  strong {
    font-weight: bold;
  }
</style>