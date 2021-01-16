<!-- App.svelte -->
<script>
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
 import White from "./c/White.svelte";
 import Alien from "./c/Alien.svelte";

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

 let m = { x: 300, y: 300 };

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

 {#if m.x > 50 && m.y > 50}
  <White {m} />
  <Alien {m} />
 {/if}
</main>
