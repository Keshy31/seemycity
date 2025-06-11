<script lang="ts">
  export let title: string;
  export let subtitle: string;
</script>

<header class="page-header">
  <h1>{title}</h1>
  <p>{subtitle}</p>
</header>

<style lang="scss">
  .page-header {
    h1 {
      font-family: var(--font-family-heading);
      font-size: var(--font-size-xxl);
      font-weight: var(--font-weight-bold);
      color: var(--text-heading-color);
      margin: 0 0 0.25em;
    }

    p {
      font-size: var(--font-size-lg);
      color: var(--text-muted-color);
      margin: 0;
      max-width: 60ch;
    }
  }
</style>
