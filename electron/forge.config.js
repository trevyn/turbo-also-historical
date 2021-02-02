module.exports = {
 packagerConfig: {
  // asar: true,
  // darwinDarkModeSupport: "true",
  // icon: "electron-app/resources/icon",
  // name: "Your app name",
  osxSign: {
   entitlements: "electron/entitlements.plist",
   "entitlements-inherit": "electron/entitlements.plist",
   "gatekeeper-assess": false,
   hardenedRuntime: true,
   // identity: "Developer ID Application: YOUR NAME HERE (YOUR ID HERE)",
  },
  osxNotarize: {
   appleId: process.env["APPLE_ID"],
   appleIdPassword: process.env["APPLE_ID_PASSWORD"],
  },
 },
 makers: [
  {
   name: "@electron-forge/maker-squirrel",
   config: {
    name: "Turbo",
   },
  },
  {
   name: "@electron-forge/maker-zip",
   platforms: ["darwin"],
  },
  {
   name: "@electron-forge/maker-deb",
   config: {},
  },
  {
   name: "@electron-forge/maker-rpm",
   config: {},
  },
 ],
};
