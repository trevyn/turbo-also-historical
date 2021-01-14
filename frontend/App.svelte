<!-- App.svelte -->
<script lang="ts">
 import { initClient, mutation } from "@urql/svelte";
 initClient({ url: "http://localhost:8080/graphql" });

 import { operationStore, query } from "@urql/svelte";
 const listPdfs = operationStore(`
    query {
      listPdfs {
        rowid
        name
      }
    }
  `);
 query(listPdfs);

 const addPdf = operationStore(`
mutation addPdf($content: String!) {
 addPdf(content: $content)
 {
  rowid
  id
  filesize
  name
  content
 }
}
  `);

 const mutateTodo = mutation(addPdf);
 function updateTodo(newTitle) {
  mutateTodo({ content: newTitle });
 }

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

  <button
   on:click={() => {
    mutateTodo({ content: 'NEW PDF' });
   }}>BUTTON</button>

  {#if $listPdfs.fetching}
   <p>Loading...</p>
  {:else if $listPdfs.error}
   <p>Oh no... {$listPdfs.error.message}</p>
  {:else}
   <ul>
    {#each $listPdfs.data.listPdfs as todo}
     <li>{JSON.stringify(todo)}</li>
    {/each}
   </ul>
  {/if}

  <a
   class="App-link"
   href="https://svelte.dev"
   target="_blank"
   rel="noopener noreferrer">
   Learn Svelte
  </a>
 </header>
</div>
