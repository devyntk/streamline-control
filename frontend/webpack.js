const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const dist = path.resolve("dist");

module.exports = {
    entry: {
        index: "./js/index.js"
    },
    output: {
        publicPath: '/dist/',
        path: dist,
        filename: "[name].js"
    },
    resolve: {
        extensions: [".scss", ".js", ".wasm"],
        alias: {
            crate: __dirname
        }
    },
    module: {
        rules: [{
            test: /\.scss$/,
            use: [
                // Extract and save the final CSS.
                MiniCssExtractPlugin.loader,
                // Translates CSS into CommonJS
                {
                    loader: 'css-loader', // translates CSS into CommonJS
                    options: {
                        importLoaders: 1,
                        url: false
                    }
                },
                {
                    loader: "postcss-loader"
                },
                // Compiles Sass to CSS
                {
                    loader: 'sass-loader',
                    options: {
                        sourceMap: true,
                    }
                }
            ]
        }]
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
            args: "--quiet",
            pluginLogLevel: 'error'
        }),
        new CleanWebpackPlugin(),
        new MiniCssExtractPlugin()
    ],
    performance: {
        // Don't break compilation because of WASM file bigger than 244 KB.
        hints: false
    },
    experiments: {
        asyncWebAssembly: true,
    },
    watchOptions: {
        ignored: ['pkg/**', 'node_modules/**']
    }
};
