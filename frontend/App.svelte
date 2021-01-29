<script>
 import { initTurboClient } from "./src/TurboClient.svelte";
 import Button from "./c/Button.svelte";
 import Card from "./c/Card.svelte";

 import {
  listCardsFullQuery,
  addCardMutation,
  cardStreamSubscription,
  deleteCardMutation,
 } from "./graphql-codegen.svelte";

 initTurboClient();

 const listCardsFull = listCardsFullQuery();
 const addCard = addCardMutation();
 const deleteCard = deleteCardMutation();
 const cardStream = cardStreamSubscription((messages = [], data) => [
  data.cardStream,
  ...messages,
 ]);

 // import the core component
 import ProsemirrorEditor from "prosemirror-svelte";

 // import helpers to work with prosemirror state
 import { createMultiLineEditor, toPlainText } from "prosemirror-svelte/state";

 // create the initial editor state
 let editorState = createMultiLineEditor("Hello world!");

 function handleChange(event) {
  // get the new editor state from event.detail
  editorState = event.detail.editorState;
 }

 // log the text content of the editor state, just for fun
 $: console.log(toPlainText(editorState));
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
  {#each $listCardsFull.data.listCardsFull as card}
   <Card
    {card}
    on:delete={(event) => {
     deleteCard({ rowid: event.detail.rowid });
     $listCardsFull.context = { requestPolicy: 'cache-and-network', forceUpdate: Date.now() };

     // alert(`AAAHHHHHHHH ${event.detail.rowid}`);
    }}>
    <ProsemirrorEditor
     placeholder="Go ahead and type something"
     {editorState}
     on:change={handleChange} />
   </Card>
  {/each}
 </ul>
{/if}
