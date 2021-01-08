const path = require("path");
const WebpackBar = require("webpackbar");
const { CleanWebpackPlugin } = require("clean-webpack-plugin");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

// Webpack generates `css_classes.rs` with this config.
// This config is used in command `yarn generate:css_classes`.
// See `webpack.config.js` for more info about individual settings.

module.exports = (env, argv) => {
    return {
        entry: path.resolve(__dirname, "./js/index.css_classes.js"),
        output: {
            path: path.resolve(__dirname, "dist"),
            filename: "css_classes.js",
        },
        plugins: [
            new WebpackBar(),
            new CleanWebpackPlugin(),
            new MiniCssExtractPlugin({
                filename: '[name].css',
                chunkFilename: '[id].css',
            })
        ],
        module: {
            rules: [
                {
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
                        "sass-loader"
                    ],
                },
            ],
        },
    };
};
