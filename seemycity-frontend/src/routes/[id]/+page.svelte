<script lang="ts">
  import { page } from '$app/state'; // Import page from $app/state (Correct for Svelte 5)

  // Access the 'id' parameter from the URL
  $: id = page.params.id;
</script>

<h1>Municipality Details</h1>

<p>Showing details for municipality ID: <strong>{id}</strong></p>

<p>(This is a placeholder page)</p>

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