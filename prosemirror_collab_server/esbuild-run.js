let onResolvePlugin = {
 name: "example",
 setup(build) {
  let path = require("path");

  // Redirect all paths starting with "frontend/" to "../frontend/"
  build.onResolve({ filter: /^frontend\// }, args => {
   return { path: path.join(args.resolveDir, "../../", args.path) };
  });
 },
};

require("esbuild")
 .build({
  entryPoints: ["src/prosemirror-collab-server.js"],
  bundle: true,
  outfile: "build/prosemirror-collab-server.js",
  plugins: [onResolvePlugin],
 })
 .catch(() => process.exit(1));
