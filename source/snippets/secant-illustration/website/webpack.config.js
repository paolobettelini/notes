const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: {
    index: "./www/index.js",
  },
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].bundle.js",
  },
  mode: "production",
  plugins: [
    new CopyWebpackPlugin({
      patterns: [ "www" ],
    })
  ],
  experiments: {
    asyncWebAssembly: true
  }
};