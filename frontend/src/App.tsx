import React from "react";
import logo from "./logo.svg";
import "./App.css";
import { Document, Page } from "react-pdf";

import { pdfjs } from "react-pdf";
console.log(`pdfjs version: ${pdfjs.version}`);
pdfjs.GlobalWorkerOptions.workerSrc = `https://cdnjs.cloudflare.com/ajax/libs/pdf.js/${pdfjs.version}/pdf.worker.min.js`;

// @mark frontend

function App() {
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

export default App;
