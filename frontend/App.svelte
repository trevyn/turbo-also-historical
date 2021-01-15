<!-- App.svelte -->
<script lang="ts">
 import {
  initClient,
  defaultExchanges,
  subscriptionExchange,
  query,
  mutation,
  subscription,
  operationStore,
 } from "@urql/svelte";
 import { SubscriptionClient } from "subscriptions-transport-ws";

 const subscriptionClient = new SubscriptionClient(
  "ws://localhost:8080/subscriptions",
  {
   reconnect: true,
  }
 );

 initClient({
  url: "http://localhost:8080/graphql",
  exchanges: [
   // devtoolsExchange,
   ...defaultExchanges,
   subscriptionExchange({
    forwardSubscription(operation) {
     return subscriptionClient.request(operation);
    },
   }),
  ],
 });

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

 const usersSubscription = operationStore(`
subscription {
 usersSubscription
 {
  id
 }
}
  `);

 const handleSubscription = (messages = [], data) => {
  console.log(data);
  return [data.usersSubscription, ...messages];
 };
 subscription(usersSubscription, handleSubscription);

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
 <p class="text-yellow-500 font-extrabold">
  Page has been here for
  <code>{count}</code>
  seconds.
 </p>

 {#if !$usersSubscription.data}
  <p>No new messages</p>
 {:else}
  <ul>
   {#each $usersSubscription.data as message}
    <li>{JSON.stringify(message)}</li>
   {/each}
  </ul>
 {/if}

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
</div>
