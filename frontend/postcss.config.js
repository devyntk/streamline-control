module.exports = (api) => {
    return {
        plugins: [
            require("postcss-typed-css-classes")({
                generator: "rust",
                purge: api.mode === "production",
                output_filepath: "src/generated/css_classes.rs",
                content: [
                    {path: ['src/**/*.rs']}
                ],
            }),
            ...(api.mode === "production" ? [require("autoprefixer")] : []),
            ...(api.mode === "production" ? [require('cssnano')({
                preset: ['default', {
                    discardComments: {
                        removeAll: true,
                    },
                }]
            })] : []),

        ]
    }
}
