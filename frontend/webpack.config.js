const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve("dist");

module.exports = {
    mode: "production",
    entry: {
        index: "./js/index.js"
    },
    output: {
        publicPath: '/dist/',
        path: dist,
        filename: "[name].js"
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
    ],
    performance: {
        // Don't break compilation because of WASM file bigger than 244 KB.
        hints: false
    },
};