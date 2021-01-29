<script>
 import { createEventDispatcher } from "svelte";
 const dispatch = createEventDispatcher();

 export let card;

 import ProsemirrorEditor from "prosemirror-svelte";
 import { createRichTextEditor, toHTML } from "prosemirror-svelte/state";

 let editorState = createRichTextEditor(card.content);

 // $: console.log(toPlainText(editorState));
</script>

<li class="col-span-1 bg-white rounded-lg shadow divide-y divide-gray-200">
 <div class="w-full flex justify-center p-6 space-x-6">
  <div class="flex-1 text-gray-500 text-sm prose">
   <ProsemirrorEditor
    placeholder="Go ahead and type something"
    {editorState}
    on:change={(event) => {
     editorState = event.detail.editorState;
     dispatch('change', toHTML(editorState));
    }} />
  </div>
 </div>
 <div>
  <div class="-mt-px flex divide-x divide-gray-200">
   <div class="w-0 flex-1 flex">
    <a
     href="#"
     on:click={() => dispatch('delete', { rowid: card.rowid })}
     class="relative -mr-px w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border border-transparent round;ed-bl-lg hover:bg-gray-50">
     <span class="ml-3">Delete</span>
    </a>
   </div>
   <div class="-ml-px w-0 flex-1 flex">
    <a
     href="#"
     class="relative w-0 flex-1 inline-flex items-center justify-center py-4 text-sm text-gray-700 font-medium border border-transparent rounded-br-lg hover:bg-gray-50">
     <span class="ml-3" />
    </a>
   </div>
  </div>
 </div>
</li>
