<script>
 import { initTurboClient } from "./src/TurboClient.svelte";

 import {
  listPdfsQuery,
  addPdfMutation,
  usersStreamSubscription,
 } from "./graphql-codegen.svelte";

 initTurboClient();

 const listPdfs = listPdfsQuery();
 const addPdf = addPdfMutation();
 const usersStream = usersStreamSubscription((messages = [], data) => [
  data.usersStream,
  ...messages,
 ]);
</script>

{#if !$usersStream.data}
 <p>No new messages</p>
{:else}
 <ul>
  {#each $usersStream.data as message}
   <li>{JSON.stringify(message)}</li>
  {/each}
 </ul>
{/if}

<button
 on:click={() => {
  addPdf({ content: 'NEW PDF' });
  $listPdfs.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
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
