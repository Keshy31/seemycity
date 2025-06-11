<script lang="ts">
  import type { PageData } from './$types';
  import MapComponent from '$lib/components/MapComponent.svelte';
  import PageHeader from '$lib/components/ui/PageHeader.svelte';
  import SearchBar from '$lib/components/ui/SearchBar.svelte';
  import ErrorMessage from '$lib/components/ui/ErrorMessage.svelte';
  import Icon from '@iconify/svelte';
  import MunicipalityDetailCard from '$lib/components/detail/MunicipalityDetailCard.svelte';

  import type { MunicipalitySearchResult } from '$lib/types';

  export let data: PageData;

  let searchQuery = '';
  let selectedMuniId: string | null = null; // To store the ID of the clicked municipality

	// The list of municipalities for the search is now derived from the GeoJSON features.
	$: municipalities =
		data.municipalityGeoJSON?.features.map(
			(feature) => feature.properties as MunicipalitySearchResult
		) ?? [];

	$: filteredMunicipalities = municipalities
		? municipalities.filter((muni: MunicipalitySearchResult) =>
				searchQuery &&
				(muni.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
					muni.id.toLowerCase().includes(searchQuery.toLowerCase()))
		  )
		: [];

  function handleSearch(event: CustomEvent<string>) {
    searchQuery = event.detail;
    selectedMuniId = null; // Clear selection when starting a new search
  }

  function handleMunicipalityClick(event: CustomEvent<{ id: string }>) {
    selectedMuniId = event.detail.id;
    searchQuery = ''; // Clear search when a muni is clicked on the map
  }

  function clearSelection() {
    selectedMuniId = null;
  }
</script>

<svelte:head>
  <title>SeeMyCity | South African Municipal Finance</title>
  <meta
    name="description"
    content="Explore the financial health of South African municipalities with an interactive map."
  />
</svelte:head>

<div class="map-view-layout">
  <aside class="sidebar">
    <PageHeader
      title="Explore the Map"
      subtitle="Find a municipality to see its financial health score"
    />

    <div class="sidebar-content">
      <SearchBar bind:value={searchQuery} on:search={handleSearch} />

      {#if data.error}
        <ErrorMessage message={data.error} />
      {:else if selectedMuniId}
        <div class="detail-view">
          <button on:click={clearSelection} class="back-button">
            <Icon icon="mdi:arrow-left" />
            <span>Back to Search</span>
          </button>
          <MunicipalityDetailCard id={selectedMuniId} />
        </div>
      {:else if searchQuery}
        <div class="search-results">
          {#if filteredMunicipalities.length > 0}
            <ul>
              {#each filteredMunicipalities as muni (muni.id)}
                <li>
                  <button class="result-item-button" on:click={() => (selectedMuniId = muni.id)}>
                    <span class="result-name">{muni.name}</span>
                    <span class="result-id">{muni.id}</span>
                  </button>
                </li>
              {/each}
            </ul>
          {:else}
            <div class="no-results">
              <p>No results found for "{searchQuery}".</p>
            </div>
          {/if}
        </div>
      {:else}
        <div class="placeholder">
          <p>Search for a municipality or click one on the map to see details here.</p>
        </div>
      {/if}
    </div>
  </aside>

  <main class="map-container">
          {#if data.municipalityGeoJSON}
        <MapComponent geojson={data.municipalityGeoJSON} on:municipalityClick={handleMunicipalityClick} />
      {/if}
  </main>
</div>

<style lang="scss">
  @use '../styles/variables' as *;

  .map-view-layout {
    display: grid;
    grid-template-columns: 380px 1fr; // Fixed sidebar, flexible map
    height: 100%; // Occupy full viewport height
    width: 100%;
    overflow: hidden; // Prevent page scroll
  }

  .sidebar {
    background-color: var(--background-color);
    display: flex;
    flex-direction: column;
    padding: var(--spacing-lg);
    border-right: 1px solid var(--border-color);
    overflow-y: auto; // Allow sidebar to scroll if content overflows
    gap: var(--spacing-xl);
  }

  .sidebar-content {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .map-container {
    position: relative; // Needed for map controls
    background-color: var(--background-offset-color); // A light background for the map area
  }

  .placeholder,
  .no-results {
    padding: var(--spacing-xl);
    text-align: center;
    background-color: var(--background-offset-color);
    border-radius: var(--border-radius-lg);
    border: 1px solid var(--border-color-light);
    color: var(--text-muted-color);
    font-size: var(--font-size-sm);
    line-height: 1.5;
  }

  .search-results {
    ul {
      list-style: none;
      padding: 0;
      margin: 0;
      display: flex;
      flex-direction: column;
      gap: var(--spacing-xs);
    }

    .result-item-button {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: var(--spacing-sm) var(--spacing-md);
      border-radius: var(--border-radius-md);
      text-decoration: none;
      color: var(--text-color);
      transition: background-color 0.2s ease;
      width: 100%;
      background: none;
      border: none;
      text-align: left;
      cursor: pointer;

      &:hover {
        background-color: var(--background-offset-color);
        color: var(--primary-color);
      }
    }

    .result-name {
      font-weight: 500;
    }

    .result-id {
      font-family: var(--font-family-mono);
      font-size: var(--font-size-xs);
      color: var(--text-muted-color);
      background-color: var(--background-offset-color);
      padding: 2px 6px;
      border-radius: var(--border-radius-sm);
      border: 1px solid var(--border-color-light);
    }
  }

  .detail-view {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);

    .back-button {
      display: inline-flex;
      align-items: center;
      gap: var(--spacing-sm);
      align-self: flex-start;
      background-color: transparent;
      color: var(--text-muted-color);
      border: none;
      padding: var(--spacing-xs) 0;
      border-radius: var(--border-radius-md);
      cursor: pointer;
      font-size: var(--font-size-sm);
      font-weight: 600;
      transition: color 0.2s ease;

      &:hover {
        color: var(--primary-color);
      }
    }
  }
</style>
