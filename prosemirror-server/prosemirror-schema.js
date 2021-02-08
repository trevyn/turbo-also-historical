import { Schema } from "prosemirror-model";
import { schema as baseSchema } from "prosemirror-schema-basic";
import { addListNodes } from "prosemirror-schema-list";

export default new Schema({
 nodes: addListNodes(baseSchema.spec.nodes, "paragraph block*", "block"),
 marks: baseSchema.spec.marks,
});
