import React from "react";
import Dropzone from "react-dropzone";
import logo from "./logo.svg";
import "./App.css";
import { Document, Page } from "react-pdf";

import { Client, defaultExchanges, subscriptionExchange, Provider } from "urql";
import { devtoolsExchange } from "@urql/devtools";
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

// SubscriptionHandler<T, R> = (prev: R | undefined, data: T) => R;

// SubscriptionHandler<UsersSubscriptionSubscription, TData>

function MyApp() {
 codegen.useUsersSubscriptionSubscription(
  {},
  (_: void, data: codegen.UsersSubscriptionSubscription) => {
   console.log("got subscription data", data.usersSubscription);
  }
 );

 const [{ data }, reload] = codegen.useListPdfsQuery();

 const [, addPdf] = codegen.useAddPdfMutation();
 const [, deletePdf] = codegen.useDeletePdfMutation();

 console.log(data);

 return (
  <div className="App">
   <header className="App-header">
    <img src={logo} className="App-logo" alt="logo" />
    <Dropzone
     onDrop={acceptedFiles =>
      acceptedFiles.forEach(file => {
       const reader = new FileReader();

       reader.onabort = () => console.log("file reading was aborted");
       reader.onerror = () => console.log("file reading has failed");
       reader.onload = () => {
        const result = reader.result;
        if (result) {
         console.log(`len is ${result.toString().length}`);
         addPdf({ content: `${result.toString()}` }).then(
          () => {
           console.log("added");
          },
          () => {
           console.log("error?");
          }
         );
        }
       };
       reader.readAsDataURL(file);
      })
     }
    >
     {({ getRootProps, getInputProps }) => (
      <section>
       <div {...getRootProps()}>
        <input {...getInputProps()} />
        <p>Drag -n- drop some files here, or click to select files</p>
       </div>
      </section>
     )}
    </Dropzone>
    <div>
     <button onClick={() => addPdf({ content: `${Date.now()}` })}>Add PDF</button>
     <button onClick={() => reload({ requestPolicy: "network-only" })}>Reload</button>
     {data &&
      data.listPdfs.map((item, i) => (
       <div key={i}>
        <button onClick={() => deletePdf({ rowid: item.rowid })}>delete {item.rowid}</button>
        {JSON.stringify({ ...item, content: undefined })}
       </div>
      ))}
     Edit <code>src/App.tsx</code> and save to reload.
    </div>
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
