/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
 mount: {
  public: "/",
  src: "/dist",
 },
 plugins: ["@snowpack/plugin-svelte"],
 optimize: {
  bundle: true,
 },
 devOptions: {
  port: 8085,
 },
};
