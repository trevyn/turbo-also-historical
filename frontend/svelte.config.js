module.exports = {
 preprocess: [
  require("svelte-preprocess").typescript(),
  require("svelte-windicss-preprocess").preprocess({
   compile: false,
   config: "tailwind.config.js",
   globalUtility: true,
   globalPreflight: true,
  }),
 ],
};
