const path = require("path");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const dist = path.resolve("dist");

module.exports = (env, argv) => {
    return {
        mode: "production",
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
                        loader: "postcss-loader",
                        options: {
                            config: {
                                // Path to postcss.config.js.
                                path: __dirname,
                                // Pass mode into `postcss.config.js` (see more info in that file).
                                ctx: { mode: argv.mode }
                            }
                        }
                    },
                    // Compiles Sass to CSS
                    "sass-loader"
                ]
            }]
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, ".")
            }),
            new CleanWebpackPlugin(),
            new MiniCssExtractPlugin({
                filename: '[name].css',
            })
        ],
        performance: {
            // Don't break compilation because of WASM file bigger than 244 KB.
            hints: false
        },
        optimization: {
            splitChunks: {
                cacheGroups: {
                    styles: {
                        name: 'styles',
                        test: /\.css$/,
                        chunks: 'all',
                        enforce: true,
                    },
                },
            },
        },
    }
};
