var fs = require("fs");

module.exports = {
 plugin: (schema, documents, config) => {
  const types = JSON.parse(fs.readFileSync(config.schemaJsonFile, "utf8")).__schema.types;

  let typeFields = {};

  types.map(type => {
   typeFields[type.name] = type.fields?.map(field => field.name);
  });

  let out = types
   .map(type => {
    switch (type.name) {
     case "Query":
     case "Subscription":
     case "Mutations":
      let queries = type.fields.map(field => {
       let name = field.name;
       let params1 = field.args.map(arg => {
        return `$${arg.name}: ${arg.type.ofType.name}!`;
       });
       let params2 = field.args.map(arg => {
        return `${arg.name}: $${arg.name}`;
       });

       params1 = params1.length ? `(${params1.join(", ")})` : "";
       params2 = params2.length ? `(${params2.join(", ")})` : "";

       let returnType =
        (field?.type?.ofType?.ofType?.ofType?.name ?? undefined) ||
        (field?.type?.ofType?.name ?? undefined);

       let lines = [];
       lines.push(
        `${
         type.name === "Query"
          ? "query"
          : type.name === "Subscription"
          ? "subscription"
          : "mutation"
        } ${field.name}${params1} {`
       );
       lines.push(` ${field.name}${params2}`);

       if (typeFields[returnType]) {
        lines.push(` {`);

        typeFields[returnType].forEach(member => {
         lines.push(`  ${member}`);
        });
        lines.push(` }`);
       } else {
        // lines.push(`  ${returnType}`);
       }

       // lines.push(` }`);
       lines.push(`}`);

       return lines.join("\n");
      });

      return queries.join("\n\n") + "\n\n";

     default:
    }
    return "";
   })
   .join("");

  return out;
 },
};
