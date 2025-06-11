<script lang="ts">
  import { onMount, createEventDispatcher } from 'svelte';
  import maplibregl, { Map, NavigationControl } from 'maplibre-gl';

  const dispatch = createEventDispatcher();

  let mapContainer: HTMLElement;
  let map: Map;

  onMount(() => {
    const apiKey = import.meta.env.VITE_MAPTILER_API_KEY;
    const styleUrl = `https://api.maptiler.com/maps/dataviz/style.json?key=${apiKey}`;

    map = new Map({
      container: mapContainer,
      style: styleUrl,
      center: [24.5, -28.8],
      zoom: 4.5
    });

    map.addControl(new NavigationControl({}), 'top-right');

    map.on('load', () => {
      const apiUrl = import.meta.env.VITE_API_BASE_URL || 'http://localhost:4000';
      map.addSource('municipalities', {
        type: 'geojson',
        data: `${apiUrl}/api/boundaries` // Assumes this endpoint returns GeoJSON
      });

      // Add a layer for the municipality fills
      map.addLayer({
        id: 'municipalities-fill',
        type: 'fill',
        source: 'municipalities',
        paint: {
          'fill-color': 'rgba(0, 128, 128, 0.2)', // Teal with some transparency
          'fill-outline-color': 'rgba(0, 128, 128, 1)'
        }
      });

      // Add a layer for the outlines
      map.addLayer({
        id: 'municipalities-outline',
        type: 'line',
        source: 'municipalities',
        paint: {
          'line-color': 'rgba(0, 128, 128, 0.8)',
          'line-width': 1
        }
      });

      // Handle clicks on the municipalities layer
      map.on('click', 'municipalities-fill', (e) => {
        if (e.features && e.features.length > 0) {
          const feature = e.features[0];
          const muniId = feature.properties.id; // Assuming the GeoJSON has an 'id' property
          if (muniId) {
            console.log(`Clicked municipality ID: ${muniId}`);
            dispatch('municipalityClick', { id: muniId });
          }
        }
      });

      // Change the cursor to a pointer when hovering over the municipalities
      map.on('mouseenter', 'municipalities-fill', () => {
        map.getCanvas().style.cursor = 'pointer';
      });

      map.on('mouseleave', 'municipalities-fill', () => {
        map.getCanvas().style.cursor = '';
      });
    });

    return () => {
      map.remove();
    };
  });
</script>

<svelte:head>
  <link href="https://unpkg.com/maplibre-gl/dist/maplibre-gl.css" rel="stylesheet" />
</svelte:head>

<div class="map-container-full" bind:this={mapContainer} />

<style lang="scss">
  .map-container-full {
    width: 100%;
    height: 100%;
    position: absolute;
    top: 0;
    left: 0;
  }
</style>