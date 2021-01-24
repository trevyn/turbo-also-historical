<script lang="ts">
 import { initTurboClient } from "./src/TurboClient.svelte";

 import {
  listCardsFullQuery,
  addCardMutation,
  cardStreamSubscription,
 } from "./graphql-codegen.svelte";

 initTurboClient();

 const listCardsFull = listCardsFullQuery();
 const addCard = addCardMutation();
 const cardStream = cardStreamSubscription((messages = [], data) => [
  data.cardStream,
  ...messages,
 ]);
</script>

{#if !$cardStream.data}
 <p>No new messages</p>
{:else}
 <ul>
  {#each $cardStream.data as message}
   <li>{JSON.stringify(message)}</li>
  {/each}
 </ul>
{/if}

<button
 on:click={() => {
  addCard({ content: 'NEW PDF' });
  $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
 }}>BUTTON</button>

{#if $listCardsFull.fetching}
 <p>Loading...</p>
{:else if $listCardsFull.error}
 <p>Oh no... {$listCardsFull.error.message}</p>
{:else}
 <ul>
  {#each $listCardsFull.data.listCardsFull as todo}
   <li>{JSON.stringify(todo)}</li>
  {/each}
 </ul>
{/if}
