var addon = require("../native");
// const { ipcMain } = require("electron");

console.log(addon.hello());

const { app, BrowserWindow } = require("electron");
var puppeteer = require("puppeteer-core");

app.commandLine.appendSwitch("force_low_power_gpu");

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

app.whenReady().then(createWindow);

app.on("window-all-closed", () => {
 if (process.platform !== "darwin") {
  app.quit();
 }
});

app.on("activate", () => {
 if (BrowserWindow.getAllWindows().length === 0) {
  createWindow();
 }
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

main();
