const prod = process.env.NODE_ENV === "production";

const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  mode: prod ? "production" : "development",

  entry: { main: "./src/main/main.ts", preload: "./src/main/preload.ts" },

  output: {
    filename: "[name].js",
    path: __dirname + "/build" + "/electron",
  },

  watch: true,

  watchOptions: {
    ignored: "/node_modules/",
  },

  target: ["electron-main", "electron-preload"],
};
