<script lang="ts">
  import maplibregl from 'maplibre-gl'; // Use only default import
  import type { FeatureCollection } from 'geojson'; // Type-only import
  import type { LngLatLike, Map, StyleSpecification, MapMouseEvent } from 'maplibre-gl'; // Import types separately
  import { onMount, onDestroy, tick } from 'svelte';
  import { page } from '$app/stores'; // For accessing map style URL
  import { browser } from '$app/environment';
  import { goto } from '$app/navigation'; // Import goto

  // Unique ID for the map container
  const mapId = `map-${Math.random().toString(36).substring(2, 15)}`;

  let mapContainer: HTMLElement | undefined;
  // Destructure needed values from the default import
  const { Map: MapClass, NavigationControl: NavigationControlClass, Popup: PopupClass } = maplibregl; // Rename all conflicting variables
  let map: Map | undefined; // Define map instance variable

  // State for fetched data
  let geojsonData: FeatureCollection | null = null;
  let isLoading = true;
  let error: string | null = null;

  // Define the type for properties based on backend response
  // Match the JSON keys returned by the API (uses serde rename)
  interface MunicipalityFeatureProperties {
    id: string; // Renamed from 'id' in backend model
    name: string;
    province: string;
    financial_score: number | null; // Renamed from 'latest_score', type is number after JSON deserialization
    population: number | null;
    classification: string | null;
    // add other properties as needed
  }

  const initialCenter: [number, number] = [24.7, -29]; // Correct order: Lng, Lat
  const initialZoom = 4.5; // Slightly more zoomed in

  // Basic inline style - Plain background
  const mapStyle: StyleSpecification = {
    version: 8,
    name: 'Blank Background',
    sources: {},
    layers: [
      {
        id: 'background',
        type: 'background',
        paint: {
          'background-color': '#FDF6E3' // Use UX background color
        }
      }
    ]
  };

  // Define color scale for scores (0-100)
  const SCORE_COLORS = [
    0, 'rgba(215, 25, 28, 0.7)',    // Red (Low Score)
    25, 'rgba(253, 174, 97, 0.7)', // Orange
    50, 'rgba(255, 255, 191, 0.7)',// Yellow
    75, 'rgba(166, 217, 106, 0.7)',// Light Green
    100, 'rgba(26, 150, 65, 0.7)'  // Dark Green (High Score)
  ];
  const DEFAULT_COLOR = 'rgba(150, 150, 150, 0.5)'; // Grey for missing scores

  // Fetch data when the component mounts
  onMount(async () => {
    console.log('MapComponent onMount: Fetching data.');
    if (!browser) return; // Don't run on server

    isLoading = true;
    error = null;
    try {
      // TEMP: Limit to 5 results for faster development loading
      const response = await fetch('/api/municipalities?limit=5'); 
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      geojsonData = await response.json();
      console.log('Fetched GeoJSON data:', geojsonData);
    } catch (e: any) {
      console.error('Error fetching map data:', e);
      error = e.message || 'Failed to load map data.';
      geojsonData = null; // Clear data on error
    } finally {
      isLoading = false; // Set loading false whether success or error
    }
  });

  // Reactive statement to initialize the map *after* loading is complete
  // and the container element exists.
  $: if (browser && !isLoading && !error && mapContainer && geojsonData && !map) {
    console.log('Reactive block: Initializing map.');

    try {
      map = new MapClass({
        container: mapContainer, // Use the bound element
        style: mapStyle,
        center: initialCenter,
        zoom: initialZoom
      });

      // Add zoom and rotation controls
      map.addControl(new NavigationControlClass(), 'top-right');

      map.on('load', () => {
        console.log('Map loaded event fired');
        if (!map || !geojsonData) { // Extra checks
          console.error('Map or geojsonData not available on load event');
          return;
        }

        // Add source with fetched data
        try {
          map.addSource('municipalities-source', {
            type: 'geojson',
            data: geojsonData
          });
          console.log('API GeoJSON source added.');

          // Add layer
          map.addLayer({
            id: 'municipalities-layer',
            type: 'fill',
            source: 'municipalities-source',
            paint: {
              'fill-color': '#cccccc', // Default color before data-driven styling is applied
              'fill-opacity': 0.6,
              'fill-outline-color': '#3C2F2F', // Charcoal for outlines
            }
          });

          // Apply data-driven styling based on financial_score
          map.setPaintProperty('municipalities-layer', 'fill-color', [
            'step',
            ['get', 'financial_score'], // Get the score property
            DEFAULT_COLOR, // Default color for null/missing scores (Grey)
            ...SCORE_COLORS // Use the defined color scale
          ]);

          // --- Layer Interaction Logic ---
          // Click event listener
          map.on('click', 'municipalities-layer', (e: MapMouseEvent & { features?: any[] }) => {
            if (!e.features || e.features.length === 0) return;
            const feature = e.features[0];
            const props = feature.properties as MunicipalityFeatureProperties;
            const coordinates = e.lngLat;
            const featureId = feature.properties.id;
            const featureName = feature.properties.name || 'Unnamed Municipality';
            const featureScore = feature.properties.financial_score !== undefined ? `${feature.properties.financial_score} / 100` : 'N/A';

            const popupHTML = `
              <div>
                <h3 class="popup-nav-link" data-id="${featureId}" style="cursor: pointer; text-decoration: underline; color: var(--primary-color, #008080); margin-bottom: 5px;">${featureName}</h3>
                <p>Score: ${featureScore}</p>
                <small>ID: ${featureId}</small>
              </div>
            `;

            const popup = new PopupClass()
              .setLngLat(coordinates)
              .setHTML(popupHTML);
            
            if (map) popup.addTo(map);

            // Add event listener to the popup's content *after* it's added
            setTimeout(() => {
              const popupElem = popup.getElement();
              const header = popupElem.querySelector('.popup-nav-link');
              if (header) {
                header.addEventListener('click', () => {
                  const navId = header.getAttribute('data-id');
                  if (navId) goto(`/${navId}`);
                });
              }
            }, 0);
          });

          // Mouse enter/leave for cursor
          map.on('mouseenter', 'municipalities-layer', () => {
            if (map) map.getCanvas().style.cursor = 'pointer';
          });
          map.on('mouseleave', 'municipalities-layer', () => {
            if (map) map.getCanvas().style.cursor = '';
          });
          // --- End Layer Interaction Logic ---

        } catch (layerError) {
          console.error('Error adding source or layer:', layerError);
          error = 'Failed to add map layers.';
        }
      });

      // Use 'any' for the event type to safely access potential 'error' property
      map.on('error', (e: any) => {
        console.error('MapLibre error:', e);
        error = e.error?.message || 'An unknown map error occurred.';
      });

    } catch (initError) {
      console.error('Failed to initialize map:', initError);
      error = 'Failed to initialize map.';
    }
  }

  onDestroy(() => {
    if (map) {
      console.log('MapComponent onDestroy: Removing map.');
      map.remove(); // Clean up the map instance
      map = undefined;
    }
  });

</script>

<!-- Basic Loading/Error UI -->
{#if isLoading}
  <p>Loading map data...</p>
{:else if error}
  <p>Error loading map: {error}</p>
{:else}
  <!-- Bind the actual map container element -->
  <div id="{mapId}" bind:this={mapContainer} class="map-container"></div>
{/if}

<!-- Simple Legend Placeholder -->
<div class="legend">
  <h4>Financial Score</h4>
  <div><span class="legend-color" style="background-color: rgba(26, 150, 65, 0.7);"></span> High (100)</div>
  <div><span class="legend-color" style="background-color: rgba(166, 217, 106, 0.7);"></span></div>
  <div><span class="legend-color" style="background-color: rgba(255, 255, 191, 0.7);"></span> Mid (50)</div>
  <div><span class="legend-color" style="background-color: rgba(253, 174, 97, 0.7);"></span></div>
  <div><span class="legend-color" style="background-color: rgba(215, 25, 28, 0.7);"></span> Low (0)</div>
  <div><span class="legend-color" style="background-color: rgba(150, 150, 150, 0.5);"></span> N/A</div>
</div>

<style>
  /* Styles specific to the map container */
  .map-container {
    width: 100%;
    height: 100%; /* Fill the parent wrapper */
  }

  /* Legend Styling */
  .legend {
    position: absolute;
    bottom: 30px;
    right: 10px;
    background-color: rgba(255, 255, 255, 0.8);
    padding: 10px;
    border-radius: 5px;
    font-size: 0.8em;
    line-height: 1.5;
    z-index: 1; /* Ensure legend is above map tiles */
    color: #3C2F2F;
    box-shadow: 0 1px 3px rgba(0,0,0,0.2);
  }
  .legend h4 {
    margin: 0 0 5px 0;
    text-align: center;
  }
  .legend div {
    display: flex;
    align-items: center;
  }
  .legend-color {
    display: inline-block;
    width: 15px;
    height: 15px;
    margin-right: 5px;
    border: 1px solid #ccc;
  }
</style>