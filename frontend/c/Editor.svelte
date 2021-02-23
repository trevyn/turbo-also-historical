<script lang="ts">
 export let placeholder: string;
 export let turbocafeId: string;

 import { createEventDispatcher } from "svelte";
 const dispatch = createEventDispatcher();

 import ProsemirrorEditor from "../prosemirror-svelte/ProsemirrorEditor.svelte";

 import schema from "./prosemirror-schema";
 import { collab, receiveTransaction, sendableSteps, getVersion } from "prosemirror-collab";
 import { DOMParser, DOMSerializer } from "prosemirror-model";
 import { EditorState, TextSelection } from "prosemirror-state";
 import { EditorView } from "prosemirror-view";

 import { corePlugins } from "../prosemirror-svelte/helpers/plugins";
 import { richTextPlugins } from "../prosemirror-svelte/helpers";
 // import { createRichTextEditor, toHTML, toPlainText } from "../prosemirror-svelte/state";
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

 let editorState: EditorState;

 import { operationStore, query, mutation, subscription } from "@urql/svelte";
 import * as gql from "../graphql-codegen";

 const getQuery = query(operationStore(gql.GetDocument, { key: turbocafeId }));

 let view: EditorView;

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
</script>

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

  let steps = sendableSteps(editorState)?.steps;
  dispatch("changecontent", JSON.stringify(steps.map(s => s.toJSON())));
  view.dispatch(receiveTransaction(editorState, steps, [999]));

  console.log(JSON.stringify(sendableSteps(editorState)?.steps.map(s => s.toJSON())));
 }}
/>
