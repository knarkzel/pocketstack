<script lang="ts">
  let counter = 0;
</script>

<main>
  <h1>Counter: {counter}</h1>
  <button on:click={() => counter++}>Increase</button>
  <button on:click={() => counter--}>Decrease</button>
</main>
