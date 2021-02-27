const addon = require("../../../native");
import { autoUpdater, app, shell, BrowserWindow } from "electron";
import puppeteer from "puppeteer-core";
import isDev from "electron-is-dev";
import { applySteps } from "./prosemirror-collab-server/prosemirror-collab-server";

app.commandLine.appendSwitch("force_low_power_gpu");

let v = `app version is ${app.getVersion()}, dev is ${isDev}`;
console.log(v);
addon.rustLog(v);

app.on("before-quit", event => {
 console.log("Caught `before-quit`.");
 addon.rustLog("Caught `before-quit`.");

 // event.preventDefault();

 // setTimeout(() => {
 //  process.exit(0);
 // }, 5000);
});

applySteps(() => {}, 0, "1", "2");

console.log(addon.hello());
console.log(addon.registerProsemirrorApplyCallback(applySteps));
console.log(addon.hello());

function initAutoUpdater() {
 // https://update.electronjs.org/trevyn/turbo/darwin-x64/0.0.0

 autoUpdater.setFeedURL({
  url: `https://update.electronjs.org/trevyn/turbo/darwin-x64/${app.getVersion()}`,
 });

 autoUpdater.on("error", err => {
  console.log("updater error");
  addon.rustLog("updater error");
  console.log(err);
 });

 autoUpdater.on("checking-for-update", () => {
  console.log("checking-for-update");
  addon.rustLog("checking-for-update");
 });

 autoUpdater.on("update-available", () => {
  console.log("update-available; downloading...");
  addon.rustLog("update-available; downloading...");
 });

 autoUpdater.on("update-not-available", () => {
  console.log("update-not-available");
  addon.rustLog("update-not-available");
 });

 autoUpdater.on("update-downloaded", (event, releaseNotes, releaseName, releaseDate, updateURL) => {
  console.log("update-downloaded", [event, releaseNotes, releaseName, releaseDate, updateURL]);
  addon.rustLog("update-downloaded");
 });

 autoUpdater.checkForUpdates();
}

function createWindow() {
 const win = new BrowserWindow({
  width: 800,
  height: 600,
  webPreferences: {
   // nodeIntegration: true,
   sandbox: true,
   contextIsolation: true,
   preload: "/Users/eden/Desktop/preload.js",
  },
 });

 // win.loadFile("index.html");
 win.loadURL("http://localhost:8080");
 // win.loadURL("https://github.com");
 win.webContents.on("did-finish-load", () => {
  win.webContents.send("ping", "whoooooooh!");
 });
}

app.whenReady().then(() => {
 shell.openExternal(isDev ? "http://localhost:8085" : "http://localhost:8080");

 if (!isDev) initAutoUpdater();
});

// app.on("window-all-closed", () => {
//  if (process.platform !== "darwin") {
//   app.quit();
//  }
// });

app.on("activate", () => {
 // if (BrowserWindow.getAllWindows().length === 0) {
 shell.openExternal(isDev ? "http://localhost:8085" : "http://localhost:8080");
 // createWindow();
 // }
});

async function main() {
 console.log("pup start");
 // @mark pup

 // https://medium.com/@jaredpotter1/connecting-puppeteer-to-existing-chrome-window-8a10828149e0
 // /Applications/Google\ Chrome.app/Contents/MacOS/Google\ Chrome --remote-debugging-port=9222

 const browser = await puppeteer.connect({
  defaultViewport: null,
  browserWSEndpoint: "ws://127.0.0.1:9222/devtools/browser/6a224b3d-ef4a-4670-ab51-bf77ece4f5fa",
 });

 const pages = await browser.pages();
 // const page = pages[0];
 let page;
 for (let i = 0; i < pages.length && !page; i++) {
  // const isHidden = await pages[i].evaluate(() => JSON.stringify(getComputedStyle(document.body)));
  // console.log((await pages[i].title()) + ", " + isHidden);
  // const isHidden = await pages[i].evaluate(() => JSON.stringify(getComputedStyle(document.body)));
  console.log(await pages[i].title());

  // if (!isHidden) {
  //  page = pages[i];
  // }
 }
 // console.log(JSON.stringify(await Promise.all(pages.map(page => page.title()))));
 // console.log(await page.title());
 // await page.goto("https://crates.io");

 // const page = await browser.newPage();
 // await page.goto("https://mozilla.github.io/pdf.js/web/viewer.html", { waitUntil: "networkidle0" });
 // await page.screenshot({ path: "screenshot.png" });

 // const browser = await puppeteer.launch({executablePath: '/path/to/Chrome', headless: false}); // default is true

 console.log("pup end");
}

//main();
