// import { collab } from "prosemirror-collab";
import { EditorState } from "prosemirror-state";
import { Step } from "prosemirror-transform";
import schema from "./prosemirror-schema";

Deno.core.ops();
const _newline = new Uint8Array([10]);
function print(value) {
 Deno.core.dispatchByName("op_print", Deno.core.encode(value.toString()), _newline);
}

let editorState_json = JSON.parse(
 `{"doc":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"The "},{"type":"text","marks":[{"type":"code"}],"text":"<audio>"},{"type":"text","text":" and "},{"type":"text","marks":[{"type":"code"}],"text":"<video>"},{"type":"text","text":" elements have several properties that you can [...] to."}]}]},"selection":{"type":"text","anchor":1,"head":1}}`
);
let steps = JSON.parse(
 `[{"stepType":"replace","from":70,"to":70,"slice":{"content":[{"type":"text","text":"x"}]}}]`
);

let editorState = EditorState.fromJSON({ schema }, editorState_json);
print(JSON.stringify(editorState));
let doc = Step.fromJSON(schema, steps[0]).apply(editorState.doc);
print(JSON.stringify(doc));
