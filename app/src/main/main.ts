import { app, BrowserWindow } from "electron";

const isProd = process.env.NODE_ENV === "production";

const createWindow = () => {
  const win = new BrowserWindow({
    width: 800,
    height: 600,
  });

  win.loadURL(`http://localhost:8000/index.html`);
};

app.whenReady().then(() => {
  createWindow();
});
