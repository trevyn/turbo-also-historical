import { Schema, NodeSpec } from "prosemirror-model";
import { schema as baseSchema } from "prosemirror-schema-basic";
import { addListNodes } from "prosemirror-schema-list";
import OrderedMap from "orderedmap";

// prosemirror-schema-basic .d.ts says:
// To reuse elements from this schema, extend or read from its
// `spec.nodes` and `spec.marks` [properties](#model.Schema.spec).

export default new Schema({
 nodes: addListNodes(baseSchema.spec.nodes as OrderedMap<NodeSpec>, "paragraph block*", "block"),
 marks: baseSchema.spec.marks,
});
