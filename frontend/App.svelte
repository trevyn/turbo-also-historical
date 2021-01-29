<script>
 import { initTurboClient } from "./src/TurboClient.svelte";
 import Button from "./c/Button.svelte";
 import Card from "./c/Card.svelte";

 import {
  listCardsFullQuery,
  addCardMutation,
  cardStreamSubscription,
  deleteCardMutation,
  updateCardMutation,
 } from "./graphql-codegen.svelte";

 initTurboClient();

 const listCardsFull = listCardsFullQuery();
 const addCard = addCardMutation();
 const deleteCard = deleteCardMutation();
 const updateCard = updateCardMutation();
 const cardStream = cardStreamSubscription((messages = [], data) => [
  data.cardStream,
  ...messages,
 ]);
</script>

<!-- {#if !$cardStream.data}
 <p>No new messages</p>
{:else}
 <ul>
  {#each $cardStream.data as message}
   <li>{JSON.stringify(message)}</li>
  {/each}
 </ul>
{/if} -->

<Button>Do Nothing</Button>

<Button
 on:click={() => {
  addCard({ content: 'NEW CARD' });
  $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
 }}>
 Add Card
</Button>

{#if $listCardsFull.fetching}
 <p>Loading...</p>
{:else if $listCardsFull.error}
 <p>Oh no... {$listCardsFull.error.message}</p>
{:else}
 <ul class="p-4 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
  {#each $listCardsFull.data.listCardsFull as card (card.rowid)}
   <Card
    {card}
    on:change={(newContent) => {
     console.log('newcontent: ', newContent.detail);
     updateCard({ rowid: card.rowid, content: newContent.detail });
    }}
    on:delete={(event) => {
     deleteCard({ rowid: event.detail.rowid });
     $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };

     // alert(`AAAHHHHHHHH ${event.detail.rowid}`);
    }} />
  {/each}
 </ul>
{/if}
