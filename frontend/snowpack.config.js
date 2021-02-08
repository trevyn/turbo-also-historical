/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
 mount: {
  /* ... */
 },
 plugins: [
  /* ... */
  "@snowpack/plugin-svelte",
  "@snowpack/plugin-postcss",
 ],
 exclude: [
  "**/node_modules/**/*",
  "queries-plugin.js",
  "run-codegen.mjs",
  "package.json",
  "package-lock.json",
 ],
 routes: [
  /* Enable an SPA Fallback in development: */
  // {"match": "routes", "src": ".*", "dest": "/index.html"},
 ],
 optimize: {
  bundle: true,
 },
 packageOptions: {
  // source: "remote",
 },
 devOptions: {
  port: 8085,
 },
 buildOptions: {
  /* ... */
 },
};
