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

 let editorState = createRichTextEditor(card.content);
 let answerEditorState = createRichTextEditor(card.answer);

 let revealed = false;

 if (toPlainText(answerEditorState).length === 0) revealed = true;
</script>

<li
 class="col-span-1 bg-white dark:bg-gray-900 rounded-lg shadow divide-y divide-gray-200 dark:divide-gray-800">
 <div class="w-full flex justify-center p-6 space-x-6">
  <div class="flex-1 text-gray-500 text-sm prose">
   <ProsemirrorEditor
    placeholder="Go ahead and type something"
    {editorState}
    on:transaction={(event) => {
     console.log('transaction');
     editorState = event.detail.editorState;
    }}
    on:change={(event) => {
     console.log('onchange');
     editorState = event.detail.editorState;
     dispatch('changecontent', toHTML(editorState));
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
