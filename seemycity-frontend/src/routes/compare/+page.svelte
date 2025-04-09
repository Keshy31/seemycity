<script lang="ts">
    import { page } from '$app/stores';
    import { dummyMunicipalityDetails } from '$lib/data/dummyStore';
    import type { MunicipalityDetails } from '$lib/data/dummyStore';
    import Icon from '@iconify/svelte';
  
    let ids: string[] = [];
    let muni1: MunicipalityDetails | null = null;
    let muni2: MunicipalityDetails | null = null;
  
    // Helper to parse IDs from the URL
    $: {
      const rawIds = $page.params.ids;
      if (rawIds) {
        // Assuming IDs are separated by '-vs-' or similar, adjust as needed
        const parsed = rawIds.split('-vs-'); 
        if (parsed.length === 2) {
          ids = parsed;
          muni1 = dummyMunicipalityDetails[ids[0]];
          muni2 = dummyMunicipalityDetails[ids[1]];
        } else {
          ids = [];
          muni1 = null;
          muni2 = null;
        }
      } else {
          ids = [];
          muni1 = null;
          muni2 = null;
      }
    }
  
    // Re-use the currency formatter from the single view (could be moved to a shared lib later)
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
  
  <svelte:head>
    <title>Compare Municipalities | SeeMyCity</title>
  </svelte:head>
  
  <h1 class="text-3xl font-bold mb-6">Comparing Municipalities</h1>
  
  {#if muni1 && muni2}
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
      <!-- Column 1: Metric Names -->
      <div class="font-semibold text-right pr-4">
        <p class="py-2">Municipality</p>
        <p class="py-2">Province</p>
        <p class="py-2">Population</p>
        <p class="py-2">Year</p>
        <p class="py-2 font-bold">Overall Score</p>
        <p class="py-2">Revenue</p>
        <p class="py-2">Debt</p>
        <p class="py-2">Capex</p>
        <p class="py-2">Audit Outcome</p>
        <!-- Add more rows for score breakdown if desired -->
      </div>
  
      <!-- Column 2: Municipality 1 -->
      <div class="border-l border-r px-4">
        <h2 class="text-xl font-semibold mb-2 py-2">{muni1.name}</h2>
        <p class="py-2">{muni1.province}</p>
        <p class="py-2">{muni1.population?.toLocaleString() ?? 'N/A'}</p>
        <p class="py-2">{muni1.year}</p>
        <p class="py-2 font-bold">{muni1.score ?? 'N/A'} / 100</p>
        <p class="py-2">{formatCurrency(muni1.revenue)}</p>
        <p class="py-2">{formatCurrency(muni1.debt)}</p>
        <p class="py-2">{formatCurrency(muni1.capital_expenditure)}</p>
        <p class="py-2">{muni1.audit_outcome}</p>
        <!-- Add more rows -->
      </div>
  
      <!-- Column 3: Municipality 2 -->
      <div class="px-4">
        <h2 class="text-xl font-semibold mb-2 py-2">{muni2.name}</h2>
        <p class="py-2">{muni2.province}</p>
        <p class="py-2">{muni2.population?.toLocaleString() ?? 'N/A'}</p>
        <p class="py-2">{muni2.year}</p>
        <p class="py-2 font-bold">{muni2.score ?? 'N/A'} / 100</p>
        <p class="py-2">{formatCurrency(muni2.revenue)}</p>
        <p class="py-2">{formatCurrency(muni2.debt)}</p>
        <p class="py-2">{formatCurrency(muni2.capital_expenditure)}</p>
        <p class="py-2">{muni2.audit_outcome}</p>
         <!-- Add more rows -->
      </div>
    </div>
  {:else}
    <p class="text-red-600">Could not load comparison. Please ensure the URL contains two valid municipality IDs separated by '-vs-' (e.g., /compare/CPT01-vs-JHB01).</p>
  {/if}
  
  <style>
    /* Add any specific styles for the comparison view here */
    .py-2 {
      padding-top: 0.5rem;
      padding-bottom: 0.5rem;
      min-height: 2.5rem; /* Ensure rows align even if content wraps */
      display: flex;
      align-items: center;
    }
    .text-right .py-2 {
        justify-content: flex-end;
    }
  </style>