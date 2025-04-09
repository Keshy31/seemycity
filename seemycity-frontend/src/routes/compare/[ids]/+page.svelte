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
    <div class="comparison-grid">
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
  {:else if ids.length > 0}
    <p class="text-red-600">Loading comparison data...</p> 
  {:else}
    <p class="text-red-600">Could not load comparison. Please ensure the URL contains two valid municipality IDs separated by '-vs-' (e.g., /compare/CPT01-vs-JHB01).</p>
  {/if}
  
  <style>
    .comparison-grid {
      display: grid;
      grid-template-columns: auto 1fr 1fr; /* Col 1 auto width, Cols 2 & 3 share space */
      gap: 1rem; /* Adjust gap as needed */
      margin-top: 1.5rem;
      background-color: #fff; /* White background for the table */
      padding: 1rem;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
 
    /* Style column headers (municipality names) */
    .comparison-grid h2 {
        color: var(--color-primary, #008080); /* Use primary color */
        font-size: 1.25rem; /* Slightly larger */
        border-bottom: 2px solid var(--color-primary-light, #B2DFDB); /* Accent border */
        padding-bottom: 0.5rem;
        margin-bottom: 1rem;
    }

    /* Ensure consistent vertical padding and minimum height for rows */
    .comparison-grid > div > p { /* Target paragraphs directly within grid columns */
       padding-top: 0.75rem; /* Slightly increased padding */
       padding-bottom: 0.75rem;
       min-height: 2.75rem; /* Slightly increased min-height */
       border-bottom: 1px solid #eee; /* Add subtle row separators */
       display: flex; /* Use flex again for vertical alignment within the min-height */
       align-items: center;
     }

     /* Remove border from last row in each column */
    .comparison-grid > div > p:last-child {
         border-bottom: none;
     }

     /* Right-align text in the first column (metric names) */
    .comparison-grid > div:first-child p {
        justify-content: flex-end; /* Align text to the right using flex */
        text-align: right;
        padding-right: 1rem; /* Add padding to separate from the value columns */
        color: #555; /* Slightly muted color for metric names */
     }

    /* Add subtle vertical borders between value columns */
    .comparison-grid > div:nth-child(2) {
        border-left: 1px solid #eee;
        border-right: 1px solid #eee;
        padding-left: 1rem;
        padding-right: 1rem;
    }
    .comparison-grid > div:nth-child(3) {
        padding-left: 1rem;
    }

    /* Error message styling */
    .text-red-600 { /* Simple placeholder - replace if needed */
        color: #dc2626; /* Tailwind's red-600 */
        margin-top: 1rem;
    }

    /* General text styling (assuming global styles handle font) */
    .font-semibold { font-weight: 600; }
    .font-bold { font-weight: 700; }
    .mb-6 { margin-bottom: 1.5rem; }
    .text-3xl { font-size: 1.875rem; }
    .text-xl { font-size: 1.25rem; }
    .pr-4 { padding-right: 1rem; }
    .px-4 { padding-left: 1rem; padding-right: 1rem; }
 </style>