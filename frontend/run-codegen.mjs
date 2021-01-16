import pkg from "@graphql-codegen/cli";
const { generate } = pkg;

let schema = {
 "http://localhost:8080/graphql": {
  headers: { Authorization: "Bearer foo" },
 },
};

async function run() {
 console.log("generating graphql.schema.json...");
 await generate(
  {
   schema: schema,
   overwrite: true,
   generates: {
    "./graphql.schema.json": {
     plugins: ["introspection"],
    },
   },
  },
  true
 );

 console.log("generating generated-queries.graphql...");
 await generate(
  {
   schema: schema,
   overwrite: true,
   generates: {
    "./generated-queries.graphql": {
     config: {
      schemaJsonFile: "./graphql.schema.json",
     },
     plugins: ["queries-plugin.js"],
    },
   },
  },
  true
 );

 console.log("generating src/graphql-codegen.ts...");
 await generate(
  {
   schema: schema,
   overwrite: true,
   generates: {
    "./graphql-codegen.svelte": {
     documents: "./generated-queries.graphql",
     plugins: ["typescript", "typescript-operations", "typescript-urql-svelte"],
     config: {
      skipTypename: false,
      withHOC: false,
      scalars: { i54: "number" },
     },
    },
   },
  },
  true
 );
}

run();
