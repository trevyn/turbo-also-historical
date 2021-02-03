module.exports = {
 packagerConfig: {
  // asar: true,
  // darwinDarkModeSupport: "true",
  // icon: "electron-app/resources/icon",
  // name: "Your app name",
  osxSign: {
   entitlements: "entitlements.plist",
   "entitlements-inherit": "entitlements.plist",
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
 publishers: [
  {
   name: "@electron-forge/publisher-github",
   config: {
    repository: {
     owner: "trevyn",
     name: "turbo",
    },
    prerelease: true,
   },
  },
 ],
};
