const path = require('path');
const {
    CleanWebpackPlugin
} = require('clean-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    plugins: [
        new CleanWebpackPlugin(),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
            outDir: path.resolve(__dirname, "lib"), 
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
    },
    mode: 'production'
};