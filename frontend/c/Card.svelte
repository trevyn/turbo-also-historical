<script>
 import { DateTime } from "luxon";
 import { createEventDispatcher } from "svelte";
 const dispatch = createEventDispatcher();

 export let card;

 import ProsemirrorEditor from "prosemirror-svelte";
 import { createRichTextEditor, toHTML } from "prosemirror-svelte/state";

 let editorState = createRichTextEditor(card.content);
 let answerEditorState = createRichTextEditor(card.answer);

 import { EditorState } from "prosemirror-state";

 // $: console.log(toPlainText(editorState));
</script>

<li class="col-span-1 bg-white rounded-lg shadow divide-y divide-gray-200">
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
 </div>

 <div class="w-full flex justify-center p-6 space-x-6">
  <div class="flex-1 text-gray-300 text-xs">
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
     class="cursor-pointer relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border-t border-gray-200 rounded-bl-lg hover:bg-gray-50">
     <span class="ml-3">Delete</span>
    </span>
   </div>
   <div class="-ml-px w-0 flex-1 flex">
    <span
     href="#"
     on:mousedown|capture|stopPropagation|preventDefault={() => console.log(editorState.selection
        .content()
        .content.toJSON())}
     class="cursor-pointer relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border-t border-l border-gray-200 rounded-br-lg hover:bg-gray-50">
     <span class="ml-3">Extract Selection</span>
    </span>
   </div>
  </div>
 </div>
</li>
