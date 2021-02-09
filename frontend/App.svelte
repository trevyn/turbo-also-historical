<script lang="ts">
 import "./index.css";
 import { initTurboClient } from "./src/TurboClient.svelte";
 import Button from "./c/Button.svelte";
 import Card from "./c/Card.svelte";

 import * as gql from "./graphql-codegen";
 import { operationStore, query, mutation, subscription } from "@urql/svelte";

 initTurboClient();

 const listCardsFull = query(operationStore(gql.ListCardsFullDocument));
 const addBlankCard = mutation(operationStore(gql.AddBlankCardDocument));
 const deleteCard = mutation(operationStore(gql.DeleteCardDocument));
 const recvSteps = mutation(operationStore(gql.RecvStepsDocument));
 const shuffleCards = mutation(operationStore(gql.ShuffleCardsDocument));
 // const cardStream = subscription(
 //  operationStore(gql.CardStreamDocument),
 //  (messages = [], data) => [data.cardStream, ...messages]
 // );

 let theme =
  localStorage.theme === "dark" ||
  (!("theme" in localStorage) &&
   window.matchMedia("(prefers-color-scheme: dark)").matches)
   ? "dark"
   : "light";

 $: if (theme === "dark") {
  document.documentElement.classList.add("dark");
 } else {
  document.documentElement.classList.remove("dark");
 }
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

<Button
 on:click={() => {
  localStorage.theme = 'light';
  theme = 'light';
 }}>
 Light
</Button>
<Button
 on:click={() => {
  localStorage.theme = 'dark';
  theme = 'dark';
 }}>
 Dark
</Button>

<Button
 on:click={async () => {
  await addBlankCard();
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
 <span class="text-gray-700 dark:text-gray-500">
  {$listCardsFull.data.listCardsFull.length}
  Cards
 </span>
 <ul class="p-4 grid grid-cols-1 gap-6">
  {#each $listCardsFull.data.listCardsFull as card (card.rowid)}
   <Card
    {card}
    on:changecontent={(steps) => {
     recvSteps({
      instantiationId: 'gW1agQGSbsxM4B7F5CSFGo5y2XD9JpMxoEGwrJZCvApRso',
      steps: steps.detail,
     });
    }}
    on:delete={(event) => {
     deleteCard({ rowid: event.detail.rowid });
     $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };
    }} />
  {/each}
 </ul>
{/if}
