<!-- App.svelte -->
<script lang="ts">
 import { onMount } from "svelte";
 let count = 0; // @hmr:keep
 onMount(() => {
  const interval = setInterval(() => count++, 1000);
  return () => {
   clearInterval(interval);
  };
 });
</script>

<div class="App">
 <header class="App-header">
  <p class="text-yellow-500 font-extrabold">
   Page has been here for
   <code>{count}</code>
   seconds.
  </p>

  <a
   class="App-link"
   href="https://svelte.dev"
   target="_blank"
   rel="noopener noreferrer">
   Learn Svelte
  </a>
 </header>
</div>
