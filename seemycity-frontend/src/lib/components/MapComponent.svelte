<script lang="ts">
  // src/lib/components/MapComponent.svelte
  import maplibregl from 'maplibre-gl'; // Default import
  import type { Map, MapLibreEvent } from 'maplibre-gl'; // Import Map type directly
  const { NavigationControl } = maplibregl; // Only destructure NavigationControl value now

  import 'maplibre-gl/dist/maplibre-gl.css';
  import { onMount, tick } from 'svelte';
  // Import FeatureCollection from geojson types (ensure @types/geojson is installed)
  import type { FeatureCollection } from 'geojson';

  // Unique ID for the map container
  const mapId = `map-${Math.random().toString(36).substring(2, 15)}`;

  let mapContainer: HTMLElement | undefined;
  let map: Map | undefined; // << Use the directly imported Map type

  // --- Dummy Data --- (Keep as is)
  const dummyMunicipalities: FeatureCollection = {
    type: 'FeatureCollection',
    features: [
      {
        type: 'Feature',
        geometry: {
          type: 'Polygon',
          // Example coordinates (replace with actual GeoJSON)
          coordinates: [
            [
              [28.0, -26.0], [28.1, -26.0], [28.1, -26.1], [28.0, -26.1], [28.0, -26.0]
            ]
          ]
        },
        properties: {
          id: 'MOCK01',
          name: 'Example Municipality A',
          score: 75 // Example score
        }
      },
      {
        type: 'Feature',
        geometry: {
          type: 'Polygon',
          // Another simple square
          coordinates: [
            [
              [29.0, -27.0], [29.1, -27.0], [29.1, -27.1], [29.0, -27.1], [29.0, -27.0]
            ]
          ]
        },
        properties: {
          id: 'MOCK02',
          name: 'Example Municipality B',
          score: 55 // Example score
        }
      },
      {
        type: 'Feature',
        geometry: {
          type: 'Polygon',
          // Simple square somewhere in the middle (approx)
          coordinates: [[ [24.0, -29.0], [24.5, -29.0], [24.5, -29.5], [24.0, -29.5], [24.0, -29.0] ]]
        },
        properties: {
          id: 'MOCK03',
          name: 'Mid Mock',
          score: 30 // Low score
        }
      }
    ]
  };
  // --- End Dummy Data ---

  onMount(() => {
    tick().then(() => {
        console.log('MapComponent onMount: Component mounted.');

        if (!mapContainer) {
            console.error("MapComponent onMount: mapContainer element not ready even after tick.");
            return; // Cannot proceed
        }

        // --- MAP INITIALIZATION (Moved here) ---
        console.log('MapComponent: Initializing map...');
        try {
            map = new maplibregl.Map({ // << Use maplibregl.Map constructor here
                container: mapId, // Using ID string for the container
                style: 'https://demotiles.maplibre.org/style.json', // Example style
                center: [22.9375, -30.5595], // Centered on South Africa
                zoom: 4,
            });

            // Add zoom and rotation controls to the map.
            map.addControl(new NavigationControl(), 'top-right');

            map.on('load', () => {
                console.log('Map loaded event fired');
                // Add dummy source
                try {
                    map?.addSource('municipalities', {
                        type: 'geojson',
                        data: dummyMunicipalities
                    });
                    console.log('Dummy source added.');
                } catch (error) {
                    console.error('Error adding dummy source:', error);
                }
                
                // Add dummy layer
                try {
                    map?.addLayer({
                        id: 'municipalities-layer',
                        type: 'fill',
                        source: 'municipalities',
                        layout: {},
                        paint: {
                            'fill-color': '#008080', // Teal color
                            'fill-opacity': 0.6
                        }
                    });
                    console.log('Dummy layer added.');
                } catch (error) {
                    console.error('Error adding dummy layer:', error);
                }
            });

            map.on('error', (e: MapLibreEvent) => { // Keep MapLibreEvent type here
                console.error('MapLibre error:', e);
            });

        } catch (error) {
            console.error('Failed to initialize MapLibre map:', error);
        }
        // --- END MAP INITIALIZATION ---

        // Cleanup function
        return () => {
            console.log('MapComponent cleanup: Removing map.');
            map?.remove();
            map = undefined;
        };
    });
  });

</script>

<!-- Bind the actual map container element -->
<div id="{mapId}" bind:this={mapContainer} class="map-container"></div>

<style>
  /* Styles specific to the map container */
  .map-container {
    width: 100%;
    height: 100%; /* Fill the parent wrapper */
  }
</style>