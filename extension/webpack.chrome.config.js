const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: {
    "content_scripts/index": "./src/content_scripts/index.ts",
    "background_scripts/index": "./src/background_scripts/index.ts",
    "popup/popup": "./src/popup/popup.ts",
  },
  devtool: "inline-source-map",
  module: {
    rules: [
      {
        test: /\.ts$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
      {
        test: /\.(scss)$/,
        use: [
          {
            loader: "style-loader",
          },
          {
            loader: "css-loader",
          },
          {
            loader: "postcss-loader",
            options: {
              postcssOptions: {
                plugins: () => [require("autoprefixer")],
              },
            },
          },
          {
            loader: "sass-loader",
          },
        ],
      },
    ],
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  output: {
    filename: "[name].js",
    path: path.resolve(__dirname, "dist/chrome"),
  },
  plugins: [
    // Copy the manifest.json to the dist folder
    new CopyPlugin({
      patterns: [
        { from: "./src/manifest.chrome.json", to: "manifest.json" },
        { from: "./src/popup/popup.html", to: "popup/popup.html" },
        { from: "./src/icons", to: "icons" },
      ],
    }),
  ],
};
