<script lang="ts">
 import { DateTime } from "luxon";
 import { createEventDispatcher } from "svelte";
 const dispatch = createEventDispatcher();

 import * as gql from "../graphql-codegen";

 export let card: gql.Card;

 import ProsemirrorEditor from "../prosemirror-svelte/ProsemirrorEditor.svelte";
 import {
  createRichTextEditor,
  toHTML,
  toPlainText,
 } from "../prosemirror-svelte/state";

 import schema from "./prosemirror-schema";
 import {
  collab,
  receiveTransaction,
  sendableSteps,
  getVersion,
 } from "prosemirror-collab";
 import { DOMParser, DOMSerializer } from "prosemirror-model";
 import { EditorState, TextSelection } from "prosemirror-state";

 import { corePlugins } from "../prosemirror-svelte/helpers/plugins";
 import { richTextPlugins } from "../prosemirror-svelte/helpers";

 // let editorState = createRichTextEditor(card.content);

 const parser = DOMParser.fromSchema(schema);
 const node = document.createElement("div");
 node.innerHTML = card.content;
 const doc = parser.parse(node);

 console.log(JSON.stringify(doc.toJSON()));

 let editorState = EditorState.create({
  schema,
  doc,
  plugins: [...corePlugins, ...richTextPlugins, collab({ clientID: 999 })],
 });

 console.log(JSON.stringify(editorState.toJSON()));

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

 let answerEditorState = createRichTextEditor(card.answer);

 let revealed = false;
 let view;

 if (toPlainText(answerEditorState).length === 0) revealed = true;
</script>

<li
 class="col-span-1 bg-white dark:bg-gray-900 rounded-lg shadow divide-y divide-gray-200 dark:divide-gray-800">
 <div class="w-full flex justify-center p-6 space-x-6">
  <div class="flex-1 text-gray-500 text-sm prose">
   <ProsemirrorEditor
    placeholder="Go ahead and type something"
    {editorState}
    bind:view
    on:transaction={(event) => {
     console.log('transaction', event);
     editorState = event.detail.editorState;
     console.log(JSON.stringify(sendableSteps(editorState)?.steps.map((s) =>
        s.toJSON()
       )));
    }}
    on:change={(event) => {
     console.log('onchange', event);
     editorState = event.detail.editorState;
     let steps = sendableSteps(editorState)?.steps;
     dispatch('changecontent', JSON.stringify(steps.map((s) => s.toJSON())));
     view.dispatch(receiveTransaction(editorState, steps, [999]));

     console.log(JSON.stringify(sendableSteps(editorState)?.steps.map((s) =>
        s.toJSON()
       )));
    }} />
  </div>
 </div>
 <div class="w-full flex justify-center p-6 space-x-6">
  {#if !revealed}
   <div
    on:click={() => (revealed = true)}
    class="cursor-pointer flex-1 text-gray-200 dark:text-gray-800 text-center text-lg underline prose">
    click to reveal answer
   </div>
  {:else}
   <div class="flex-1 text-gray-500 text-sm prose">
    <ProsemirrorEditor
     placeholder="Answer goes here"
     editorState={answerEditorState}
     on:transaction={(event) => {
      answerEditorState = event.detail.editorState;
     }}
     on:change={(event) => {
      answerEditorState = event.detail.editorState;
      dispatch('changeanswer', toHTML(answerEditorState));
     }} />
   </div>
  {/if}
 </div>

 <div class="w-full flex justify-center p-6 space-x-6">
  <div class="flex-1 text-gray-300 dark:text-gray-700 text-xs">
   Created
   {card.createdTime && `${DateTime.fromSeconds(card.createdTime).toRelative()} (${DateTime.fromSeconds(card.createdTime).toFormat('MMM d, yyyy')})`}<br />
   Modified
   {card.modifiedTime && `${DateTime.fromSeconds(card.modifiedTime).toRelative()} (${DateTime.fromSeconds(card.modifiedTime).toFormat('MMM d, yyyy')})`}
  </div>
 </div>

 <div>
  <div class="-mt-px flex">
   <div class="w-0 flex-1 flex">
    <span
     href="#"
     on:click={() => dispatch('delete', { rowid: card.rowid })}
     class="cursor-pointer relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 dark:text-gray-400 font-medium border-t border-gray-200 dark:border-gray-800 rounded-bl-lg hover:bg-gray-50 dark:hover:bg-gray-800">
     <span class="ml-3">Delete</span>
    </span>
   </div>
   <div class="-ml-px w-0 flex-1 flex">
    <span
     href="#"
     on:mousedown|capture|stopPropagation|preventDefault={() => console.log(editorState.selection
        .content()
        .content.toJSON())}
     class="cursor-pointer relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 dark:text-gray-400 font-medium border-t border-l border-gray-200 dark:border-gray-800 rounded-br-lg hover:bg-gray-50 dark:hover:bg-gray-800">
     <span class="ml-3">Extract Selection</span>
    </span>
   </div>
  </div>
 </div>
</li>
