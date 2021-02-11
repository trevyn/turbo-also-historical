// import { collab } from "prosemirror-collab";
import { EditorState } from "prosemirror-state";
import { Step } from "prosemirror-transform";
import schema from "../../../frontend/c/prosemirror-schema";

let editorState_json = JSON.parse(
 `{"doc":{"type":"doc","content":[{"type":"paragraph","content":[{"type":"text","text":"The "},{"type":"text","marks":[{"type":"code"}],"text":"<audio>"},{"type":"text","text":" and "},{"type":"text","marks":[{"type":"code"}],"text":"<video>"},{"type":"text","text":" elements have several properties that you can [...] to."}]}]},"selection":{"type":"text","anchor":1,"head":1}}`
);
let mySteps = JSON.parse(
 `[{"stepType":"replace","from":70,"to":70,"slice":{"content":[{"type":"text","text":"x"}]}}]`
);

export function applySteps(
 callback: (slot: number, result: string) => void,
 slot: number,
 editorState: string,
 steps: string
) {
 console.log("slot:", slot);
 console.log("in editorState:", editorState);
 console.log("in steps:", steps);

 let myEditorState = EditorState.fromJSON({ schema }, editorState_json);
 let doc = Step.fromJSON(schema, mySteps[0]).apply(myEditorState.doc);

 console.log("out doc:", JSON.stringify(doc));

 callback(slot, JSON.stringify(doc));

 console.log("callback called");
}
