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

    let muniDetails1: MunicipalityDetails | null = null;
    let muniDetails2: MunicipalityDetails | null = null;
    let isLoading = true;
    let error: string | null = null;
    let ids: string[] = [];

    onMount(async () => {
        const rawIds = $page.params.ids;
        if (!rawIds) {
            error = "Comparison IDs not found in URL.";
            isLoading = false;
            return;
        }

        const parsed = rawIds.split('-vs-');
        if (parsed.length !== 2 || !parsed[0] || !parsed[1]) {
            error = "Invalid comparison URL. Expecting two IDs separated by '-vs-' (e.g., /compare/ID1-vs-ID2).";
            isLoading = false;
            return;
        }

        ids = parsed;
        isLoading = true;
        error = null;

        try {
            const fetchMuni = async (id: string): Promise<MunicipalityDetails> => {
                const response = await fetch(`/api/municipalities/${id}`);
                if (!response.ok) {
                    if (response.status === 404) {
                        throw new Error(`Municipality with ID '${id}' not found.`);
                    } else {
                        throw new Error(`HTTP error fetching ID ${id}: ${response.status}`);
                    }
                }
                return await response.json();
            };

            const [details1, details2] = await Promise.all([
                fetchMuni(ids[0]),
                fetchMuni(ids[1])
            ]);

            muniDetails1 = details1;
            muniDetails2 = details2;
            console.log('Fetched Comparison Details:', muniDetails1, muniDetails2);

        } catch (e: any) {
            console.error('Error fetching comparison details:', e);
            error = e.message || 'Failed to load comparison data.';
            muniDetails1 = null; 
            muniDetails2 = null;
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
  
  <svelte:head>
    <title>Compare Municipalities | SeeMyCity</title>
  </svelte:head>
  
  <h1 class="text-3xl font-bold mb-6">Comparing Municipalities</h1>
  
  {#if isLoading}
      <p>Loading comparison data...</p>
  {:else if error}
      <p class="text-red-600">Error: {error}</p>
  {:else if muniDetails1 && muniDetails2}
    <div class="comparison-grid">
      <!-- Column 1: Metric Names -->
      <div class="font-semibold text-right pr-4">
        <p class="py-2">Municipality</p>
        <p class="py-2">Province</p>
        <p class="py-2">Population</p>
        <p class="py-2">Year</p>
        <p class="py-2">Revenue</p>
        <p class="py-2">Expenditure</p> 
        <p class="py-2">Debt</p> 
        <p class="py-2">Audit Outcome</p>
      </div>
  
      <!-- Column 2: Municipality 1 -->
      <div class="border-l border-r px-4">
        <h2 class="text-xl font-semibold mb-2 py-2">{muniDetails1.name}</h2>
        <p class="py-2">{muniDetails1.province}</p>
        <p class="py-2">{muniDetails1.population?.toLocaleString() ?? 'N/A'}</p>
        <p class="py-2">{muniDetails1.financial_year ?? 'N/A'}</p>
        <p class="py-2">{formatCurrency(muniDetails1.revenue)}</p>
        <p class="py-2">{formatCurrency(muniDetails1.expenditure)}</p>
        <p class="py-2">{formatCurrency(muniDetails1.debt)}</p>
        <p class="py-2">{muniDetails1.audit_outcome ?? 'N/A'}</p>
      </div>
  
      <!-- Column 3: Municipality 2 -->
      <div class="px-4">
        <h2 class="text-xl font-semibold mb-2 py-2">{muniDetails2.name}</h2>
        <p class="py-2">{muniDetails2.province}</p>
        <p class="py-2">{muniDetails2.population?.toLocaleString() ?? 'N/A'}</p>
        <p class="py-2">{muniDetails2.financial_year ?? 'N/A'}</p>
        <p class="py-2">{formatCurrency(muniDetails2.revenue)}</p>
        <p class="py-2">{formatCurrency(muniDetails2.expenditure)}</p>
        <p class="py-2">{formatCurrency(muniDetails2.debt)}</p>
        <p class="py-2">{muniDetails2.audit_outcome ?? 'N/A'}</p>
      </div>
    </div>
  {:else}
    <p class="text-red-600">Could not load comparison data. Please check the IDs.</p>
  {/if}
  
  <style>
    .comparison-grid {
      display: grid;
      grid-template-columns: auto 1fr 1fr; 
      gap: 1rem; 
      margin-top: 1.5rem;
      background-color: #fff; 
      padding: 1rem;
      border-radius: 8px;
      box-shadow: 0 2px 4px rgba(0,0,0,0.05);
    }
 
    .comparison-grid h2 {
        color: var(--color-primary, #008080); 
        font-size: 1.25rem; 
        border-bottom: 2px solid var(--color-primary-light, #B2DFDB); 
        padding-bottom: 0.5rem;
        margin-bottom: 1rem;
    }

    .comparison-grid > div > p { 
       padding-top: 0.75rem; 
       padding-bottom: 0.75rem;
       min-height: 2.75rem; 
       border-bottom: 1px solid #eee; 
       display: flex; 
       align-items: center;
     }

     .comparison-grid > div > p:last-child {
         border-bottom: none;
     }

    .comparison-grid > div:first-child p {
        justify-content: flex-end; 
        text-align: right;
        padding-right: 1rem; 
        color: #555; 
     }

    .comparison-grid > div:nth-child(2) {
        border-left: 1px solid #eee;
        border-right: 1px solid #eee;
        padding-left: 1rem;
        padding-right: 1rem;
    }
    .comparison-grid > div:nth-child(3) {
        padding-left: 1rem;
    }

    .text-red-600 { 
        color: #dc2626; 
        margin-top: 1rem;
    }

    .font-semibold { font-weight: 600; }
    .font-bold { font-weight: 700; }
    .mb-6 { margin-bottom: 1.5rem; }
    .text-3xl { font-size: 1.875rem; }
    .text-xl { font-size: 1.25rem; }
    .pr-4 { padding-right: 1rem; }
    .px-4 { padding-left: 1rem; padding-right: 1rem; }
 </style>