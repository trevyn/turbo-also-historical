<script context="module">
 import {
  initClient,
  defaultExchanges,
  subscriptionExchange,
 } from "@urql/svelte";
 import { devtoolsExchange } from "@urql/devtools";
 import { SubscriptionClient } from "subscriptions-transport-ws";

 // import {
 //  listPdfsQuery,
 //  addPdfMutation,
 //  usersSubscriptionSubscription,
 // } from "./graphql-codegen.svelte";

 const subscriptionClient = new SubscriptionClient(
  "ws://localhost:8080/subscriptions",
  {
   reconnect: true,
  }
 );

 export function initTurboClient() {
  initClient({
   url: "http://localhost:8080/graphql",
   exchanges: [
    devtoolsExchange,
    ...defaultExchanges,
    subscriptionExchange({
     forwardSubscription(operation) {
      return subscriptionClient.request(operation);
     },
    }),
   ],
  });
 }

 // const listPdfs = listPdfsQuery();
 // const addPdf = addPdfMutation();
 // const usersSubscription = usersSubscriptionSubscription(
 //  (messages = [], data) => [data.usersSubscription, ...messages]
 // );
</script>
