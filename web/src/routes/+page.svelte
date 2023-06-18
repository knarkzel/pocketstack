<script lang="ts">
  import type { PageData } from "./$types";

  export let data: PageData;
</script>

<svelte:head>
  <title>Let's get cracking bones!</title>
</svelte:head>

<h1 class="h1">Let's get cracking bones!</h1>

{#await data.streamed.message}
  <p>Loading message...</p>
{:then message}
  <p>{message}</p>
{/await}

<p>Check my <a href="/about">about page</a>.</p>
