const path = require('path');
const {CleanWebpackPlugin} = require('clean-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const OptimizeCSSAssetsPlugin = require('optimize-css-assets-webpack-plugin');
const CopyWebpackPlugin = require("copy-webpack-plugin");

const distPath = path.resolve(__dirname, 'dist');

module.exports = (env, argv) => {
    return {
        devServer: {
            contentBase: distPath,
            compress: argv.mode === 'production',
            port: 8000,
            historyApiFallback: true,
            liveReload: true,
        },
        entry: path.resolve(__dirname, 'bootstrap.js'),
        output: {
            path: distPath,
            filename: 'bundle.min.js',
            webassemblyModuleFilename: 'bundle.wasm'
        },
        module: {
            rules: [
                {
                    test: /\.js$/,
                    exclude: /node_modules/,
                    use: {
                      loader: 'babel-loader',
                      options: {
                        presets: ['@babel/preset-env']
                      }
                    }
                },
                // Loading styles
                {
                    test: /\.css$/i,
                    use: [
                        'style-loader',
                        'css-loader',
                        'sass-loader',
                    ],
                },
                // Loading images
                {
                    test: /\.(png|svg|jpg|jpeg|gif)$/i,
                    use: [{
                      loader: 'file-loader',
                      options: {
                        name: '[name].[ext]',
                        outputPath: distPath + '/public/images/'
                      }
                    }]
                },
                // Loading fonts
                {
                    test: /\.(woff(2)?|ttf|eot|svg|otf)(\?v=\d+\.\d+\.\d+)?$/,
                    // include: path.resolve(__dirname, 'public/fonts'),
                    // exclude: path.resolve(__dirname, 'public/images'),
                    use: [{
                      loader: 'file-loader',
                      options: {
                        name: '[name].[ext]',
                        outputPath: distPath + '/public/fonts/'
                      }
                    }]
                },
            ]
        },
        plugins: [
            new CleanWebpackPlugin(),
            new CopyWebpackPlugin({
                patterns: [
                  { from: "public", to: distPath },
                ],
            }),
            new MiniCssExtractPlugin({filename: 'bundle.min.css'}),
            new WasmPackPlugin({crateDirectory: '.', extraArgs: '--no-typescript',}),
            new OptimizeCSSAssetsPlugin({}),
            new HtmlWebpackPlugin({template: "index.html"}),
        ],
        watch: argv.mode !== 'production',
    };
};
