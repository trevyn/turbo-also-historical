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

 let m = { x: 0, y: 0 };

 function handleMousemove(event) {
  m.x = event.clientX;
  m.y = event.clientY;
 }
</script>

<main style="width:100%; height:100vh" on:mousemove={handleMousemove}>
 <div>The mouse position is {m.x - 300} x {m.y - 300}</div>

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

 <div
  class="m-10 absolute rounded-lg border border-gray-300 bg-white px-6 py-5 shadow-sm flex items-center space-x-3 hover:border-gray-400 focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-gray-400 max-w-md"
  style="left:{m.x - 100}px;top:{m.y - 100}px;">
  <div class="flex-shrink-0">
   <img
    class="h-20 w-20 rounded-full"
    src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=facearea&facepad=2&w=256&h=256&q=80"
    alt="" />
  </div>
  <div class="min-w-0">
   <a href="#" class="focus:outline-none">
    <span class="absolute inset-0" aria-hidden="true" />
    <p class="text-2xl font-medium text-gray-700">Leslie Alexander</p>
    <p class="text-2xl text-gray-400 truncate">Co-Founder / CEO</p>
   </a>
  </div>
 </div>
</main>
