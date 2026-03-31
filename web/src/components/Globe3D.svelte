<script>
  import { onMount, createEventDispatcher } from 'svelte';
  import Globe from 'globe.gl';

  export let cities = []; // Array of {id, name, latitude, longitude, country_name, ...}

  const dispatch = createEventDispatcher();

  let container;
  let globe;

  onMount(() => {
    // Create Globe.GL instance
    globe = Globe()
      (container)
      .globeImageUrl('//unpkg.com/three-globe/example/img/earth-blue-marble.jpg')
      .bumpImageUrl('//unpkg.com/three-globe/example/img/earth-topology.png')
      .backgroundImageUrl('//unpkg.com/three-globe/example/img/night-sky.png')
      .showAtmosphere(true)
      .atmosphereColor('lightskyblue')
      .atmosphereAltitude(0.15);

    // Configure camera
    globe.controls().autoRotate = false;
    globe.controls().enableZoom = true;

    // Set initial view
    globe.pointOfView({ lat: 20, lng: 0, altitude: 2.5 }, 1000);

    // Render city markers
    renderCities();

    // Handle resize
    window.addEventListener('resize', handleResize);

    return () => {
      window.removeEventListener('resize', handleResize);
      if (globe) {
        // Properly dispose of Globe.gl and Three.js resources
        try {
          // Access underlying Three.js components if available
          const scene = globe.scene();
          const renderer = globe.renderer();

          // Dispose of scene objects
          if (scene) {
            scene.traverse((object) => {
              if (object.geometry) {
                object.geometry.dispose();
              }
              if (object.material) {
                if (Array.isArray(object.material)) {
                  object.material.forEach(material => material.dispose());
                } else {
                  object.material.dispose();
                }
              }
            });
          }

          // Dispose renderer
          if (renderer) {
            renderer.dispose();
          }

          // Clear container
          if (container) {
            container.innerHTML = '';
          }
        } catch (e) {
          console.warn('Globe cleanup error:', e);
        }

        globe = null;
      }
    };
  });

  function handleResize() {
    if (globe) {
      globe.width(container.clientWidth);
      globe.height(container.clientHeight);
    }
  }

  function renderCities() {
    if (!globe || !cities || cities.length === 0) return;

    // Convert cities to points data for Globe.GL
    const pointsData = cities.map(city => ({
      lat: city.latitude,
      lng: city.longitude,
      size: 0.15,
      color: '#FFD700', // Gold
      city: city, // Store full city data
    }));

    // Add points layer
    globe
      .pointsData(pointsData)
      .pointAltitude(0.01)
      .pointRadius('size')
      .pointColor('color')
      .pointLabel(d => `
        <div style="background: rgba(0,0,0,0.8); padding: 8px 12px; border-radius: 6px; color: white;">
          <div style="font-weight: 600; font-size: 14px;">${d.city.name}</div>
          <div style="font-size: 12px; color: #ccc;">${d.city.country_name}</div>
        </div>
      `)
      .onPointClick(point => {
        // Dispatch city click event
        dispatch('cityClick', { city: point.city });
      })
      .onPointHover(point => {
        // Change cursor on hover
        container.style.cursor = point ? 'pointer' : 'grab';
      });
  }

  // Re-render when cities change
  $: if (globe && cities) {
    renderCities();
  }
</script>

<div bind:this={container} class="globe-container"></div>

<style>
  .globe-container {
    width: 100%;
    height: 100%;
    position: relative;
    cursor: grab;
  }

  .globe-container:active {
    cursor: grabbing;
  }

  :global(.scene-tooltip) {
    pointer-events: none;
  }
</style>
