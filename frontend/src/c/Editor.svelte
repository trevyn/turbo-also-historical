<script lang="ts">
 import * as gql from "../graphql-codegen";
 import { createEventDispatcher } from "svelte";
 const dispatch = createEventDispatcher();
 import { operationStore, query, mutation, subscription } from "@urql/svelte";
 import ProsemirrorEditor from "../prosemirror-svelte/ProsemirrorEditor.svelte";
 import { corePlugins } from "../prosemirror-svelte/helpers/plugins";
 import { richTextPlugins } from "../prosemirror-svelte/helpers";
 import { createRichTextEditor, toHTML, toPlainText } from "../prosemirror-svelte/state";
 import schema from "./prosemirror-schema";
 import { collab, receiveTransaction, sendableSteps, getVersion } from "prosemirror-collab";
 import { DOMParser, DOMSerializer } from "prosemirror-model";
 import { EditorState, TextSelection } from "prosemirror-state";
 import { EditorView } from "prosemirror-view";

 // ---

 export let placeholder: string;
 export let turbocafeId: string;

 let editorState: EditorState;
 let view: EditorView;

 const getQuery = query(operationStore(gql.GetDocument, { key: turbocafeId }));
 const putKv = mutation(operationStore(gql.PutKvDocument));

 // --- Got data, put it in the editor

 $: if ($getQuery.data) {
  const parser = DOMParser.fromSchema(schema);
  const node = document.createElement("div");
  node.innerHTML = getQuery.data.get;
  const doc = parser.parse(node);

  editorState = EditorState.create({
   schema,
   doc,
   plugins: [...corePlugins, ...richTextPlugins, collab({ clientID: 999 })],
  });
 }

 // ---

 // let editorState = createRichTextEditor(card.content);
 // console.log(JSON.stringify(editorState.toJSON()));

 // let view = new EditorView(place, {
 //  state: EditorState.create({
 //   doc: authority.doc,
 //   plugins: [collab.collab({ version: authority.steps.length })],
 //  }),
 //  dispatchTransaction(transaction) {
 //   let newState = view.state.apply(transaction);
 //   view.updateState(newState);
 //   let sendable = collab.sendableSteps(newState);
 //   if (sendable)
 //    authority.receiveSteps(sendable.version, sendable.steps, sendable.clientID);
 //  },
 // });
</script>

{#if $getQuery.error}
 <p>Oh no... {$getQuery.error.message}</p>
{:else if $getQuery.data}
 <ProsemirrorEditor
  {placeholder}
  {editorState}
  bind:view
  on:transaction={event => {
   console.log("transaction", event);
   editorState = event.detail.editorState;
   console.log(JSON.stringify(sendableSteps(editorState)?.steps.map(s => s.toJSON())));
  }}
  on:change={event => {
   console.log("onchange", event);
   editorState = event.detail.editorState;
   // dispatch("change", toHTML(editorState));
   putKv({ key: turbocafeId, value: toHTML(editorState) });

   // let steps = sendableSteps(editorState)?.steps;
   // dispatch("changecontent", JSON.stringify(steps.map(s => s.toJSON())));
   // view.dispatch(receiveTransaction(editorState, steps, [999]));

   // console.log(JSON.stringify(sendableSteps(editorState)?.steps.map(s => s.toJSON())));
  }}
 />
{/if}
