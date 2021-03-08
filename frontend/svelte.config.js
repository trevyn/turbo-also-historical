module.exports = {
 preprocess: [
  require("svelte-preprocess").typescript(),
  require("svelte-windicss-preprocess").preprocess({
   compile: false,
   config: "tailwind.config.js",
   silent: false,
   debug: true,
  }),
 ],
};
