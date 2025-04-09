<script lang="ts">
  // src/lib/components/MapComponent.svelte
  import maplibregl from 'maplibre-gl'; // Default import
  import type { Map, MapLibreEvent, GeoJSONSource, LngLatLike, Popup, MapMouseEvent } from 'maplibre-gl'; // Import Map type directly
  import type { FeatureCollection } from 'geojson'; // Import GeoJSON types
  const { NavigationControl } = maplibregl; // Only destructure NavigationControl value now

  import 'maplibre-gl/dist/maplibre-gl.css';
  import { onMount, tick } from 'svelte';
  import { page } from '$app/stores'; // For accessing map style URL
  import { browser } from '$app/environment';
  import { goto } from '$app/navigation'; // Import goto
  import { dummyMunicipalitiesGeoJSON } from '$lib/data/dummyStore'; // Import from store

  // Unique ID for the map container
  const mapId = `map-${Math.random().toString(36).substring(2, 15)}`;

  let mapContainer: HTMLElement | undefined;
  let map: Map | undefined; // << Use the directly imported Map type

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
                // Ensure map is defined within this scope for TypeScript
                if (!map) {
                    console.error("Map became undefined before 'load' event completed?");
                    return;
                }

                // Add dummy source
                try {
                    map.addSource('municipalities-source', {
                        type: 'geojson',
                        // Use the imported data
                        data: dummyMunicipalitiesGeoJSON 
                    });
                    console.log('Dummy source added.');
                } catch (error) {
                    console.error('Error adding dummy source:', error);
                }
                
                // Add dummy layer
                try {
                    map.addLayer({
                        id: 'municipalities-layer',
                        type: 'fill',
                        source: 'municipalities-source',
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

                // Click event listener for the municipalities layer
                map.on('click', 'municipalities-layer', (e: MapMouseEvent & { features?: any[] }) => {
                    console.log('Click event on municipalities-layer', e.features);
                    if (!e.features || e.features.length === 0) {
                        console.log('No features found at click point.');
                        return;
                    }

                    const feature = e.features[0];
                    const coordinates = e.lngLat;
                    const featureId = feature.properties.id; // Get the ID
                    const featureName = feature.properties.name || 'Unnamed Municipality';
                    const featureScore = feature.properties.score !== undefined ? `${feature.properties.score} / 100` : 'N/A';

                    // Create HTML content for the popup with a clickable header
                    const popupHTML = `
                        <div>
                            <h3 class="popup-nav-link" data-id="${featureId}" style="cursor: pointer; text-decoration: underline; color: var(--primary-color, #008080); margin-bottom: 5px;">${featureName}</h3>
                            <p>Score: ${featureScore}</p>
                            <small>ID: ${featureId}</small>
                        </div>
                    `;

                    // Ensure existing popups are closed (optional, good practice)
                    // closePopup(); // You might need a function to track and close existing popups if desired

                    const popup = new maplibregl.Popup()
                        .setLngLat(coordinates)
                        .setHTML(popupHTML);
                    
                    // Ensure map is defined before adding popup    
                    if (map) {
                        popup.addTo(map); 
                    } else {
                        console.error("Cannot add popup, map is undefined.");
                        return; // Exit if map is somehow undefined here
                    }
                     
                    // Use setTimeout to ensure popup DOM is ready
                    setTimeout(() => {
                        console.log('Popup opened event fired.'); // <-- Log 1
                        const popupElem = popup.getElement();
                        const header = popupElem.querySelector('.popup-nav-link');
                        if (header) {
                            // Add listener to the header specifically
                            header.addEventListener('click', () => {
                                const navId = header.getAttribute('data-id');
                                if (navId) {
                                    console.log(`Navigating to /${navId}`);
                                    goto(`/${navId}`);
                                }
                            });
                        } else {
                            console.error("Could not find '.popup-nav-link' element inside popup.");
                        }
                    }, 0); // Delay of 0 milliseconds
                });

                // Change cursor to pointer when hovering over the municipalities layer
                if (map) { // Check map before attaching listener
                    map.on('mouseenter', 'municipalities-layer', () => {
                        if (map) { // Check map again before using inside callback
                            map.getCanvas().style.cursor = 'pointer';
                        }
                    });
                }

                if (map) { // Check map before attaching listener
                    map.on('mouseleave', 'municipalities-layer', () => {
                        if (map) { // Check map again before using inside callback
                            map.getCanvas().style.cursor = '';
                        }
                    });
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