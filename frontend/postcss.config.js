module.exports = ({ file, options, env }) => {
    return {
        plugins: [
            require("postcss-typed-css-classes")({
                generator: "rust",
                purge: true,
                output_filepath: "src/generated/css_classes.rs",
                content: [
                    {path: ['src/**/*.rs']}
                ],
            }),
            require('cssnano')
        ]
    }
}
