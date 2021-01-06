import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { Document, Page } from "react-pdf";

import { Client, defaultExchanges, subscriptionExchange, Provider } from "urql";
import { SubscriptionClient } from "subscriptions-transport-ws";

import * as codegen from "./graphql-codegen";

import { pdfjs } from "react-pdf";
console.log(`pdfjs version: ${pdfjs.version}`);
pdfjs.GlobalWorkerOptions.workerSrc = `https://cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.min.js`;

// const client = createClient({
//  url: "http://localhost:8080/graphql",
// });

const subscriptionClient = new SubscriptionClient("ws://localhost:8080/subscriptions", {
 reconnect: true,
});
const client = new Client({
 url: "/graphql",
 exchanges: [
  ...defaultExchanges,
  subscriptionExchange({
   forwardSubscription(operation) {
    return subscriptionClient.request(operation);
   },
  }),
 ],
});

// const wsClient = createWSClient({
//  url: "ws://localhost:8080/subscriptions",
// });
// const client = createClient({
//  url: "/subscriptions",
//  exchanges: [
//   ...defaultExchanges,
//   subscriptionExchange({
//    forwardSubscription(operation) {
//     return {
//      subscribe: sink => {
//       const dispose = wsClient.subscribe(operation, sink);
//       return {
//        unsubscribe: dispose,
//       };
//      },
//     };
//    },
//   }),
//  ],
// });

// @mark frontend
// const handleSubscription = (messages = [], response) => {
//  return [response.newMessages, ...messages];
// };

function MyApp() {
 codegen.useUsersSubscriptionSubscription({}, () => {
  console.log("hello");
 });

 return (
  <div className="App">
   <header className="App-header">
    <img src={logo} className="App-logo" alt="logo" />
    <p>
     Edit <code>src/App.tsx</code> and save to reload.
    </p>
    <a className="App-link" href="https://reactjs.org" target="_blank" rel="noopener noreferrer">
     Learn React
    </a>
    <Document file="http://localhost:3000/compressed.tracemonkey-pldi-09.pdf">
     <Page pageNumber={1} />
     <Page pageNumber={2} />
    </Document>
   </header>
  </div>
 );
}

function App() {
 return (
  <Provider value={client}>
   <MyApp />
  </Provider>
 );
}

export default App;
