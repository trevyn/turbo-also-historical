<script lang="ts">
 import { initTurboClient } from "./src/TurboClient.svelte";
 import Button from "./c/Button.svelte";
 import Card from "./c/Card.svelte";

 import * as gql from "./graphql-codegen";
 import { operationStore, query, mutation, subscription } from "@urql/svelte";

 initTurboClient();

 const listCardsFull = query(operationStore(gql.ListCardsFullDocument));
 const addCard = mutation(operationStore(gql.AddCardDocument));
 const deleteCard = mutation(operationStore(gql.DeleteCardDocument));
 const updateCard = mutation(operationStore(gql.UpdateCardDocument));
 const shuffleCards = mutation(operationStore(gql.ShuffleCardsDocument));
 // const cardStream = subscription(
 //  operationStore(gql.CardStreamDocument),
 //  (messages = [], data) => [data.cardStream, ...messages]
 // );
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
 on:click={async () => {
  await addCard({ content: '', answer: '' });
  $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
 }}>
 Add Card
</Button>

<Button
 on:click={async () => {
  await shuffleCards();
  $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
 }}>
 Shuffle Cards
</Button>

{#if $listCardsFull.error}
 <p>Oh no... {$listCardsFull.error.message}</p>
{:else if $listCardsFull.data}
 {$listCardsFull.data.listCardsFull.length}
 Cards

 <ul class="p-4 grid grid-cols-1 gap-6">
  {#each $listCardsFull.data.listCardsFull as card (card.rowid)}
   <Card
    {card}
    on:changecontent={(newContent) => updateCard({
      rowid: card.rowid,
      content: newContent.detail,
      answer: card.answer,
     })}
    on:changeanswer={(newAnswer) => updateCard({
      rowid: card.rowid,
      content: card.content,
      answer: newAnswer.detail,
     })}
    on:delete={(event) => {
     deleteCard({ rowid: event.detail.rowid });
     $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };

     // alert(`AAAHHHHHHHH ${event.detail.rowid}`);
    }} />
  {/each}
 </ul>
{/if}
