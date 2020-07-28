const path = require('path');
const VueLoaderPlugin = require('vue-loader/lib/plugin');

module.exports = {
  mode: 'development',
  entry: './src/assets/js/app.js',
  output: {
    path: path.resolve(__dirname, './public/static'),
    filename: 'main.js'
  },
  resolve: {
    // with these, we can successfully remove .vue from vue imports.
    extensions: ['*', '.js', '.vue', '.json'],
    alias: {
      vue: 'vue/dist/vue.js',
      '@': path.resolve(__dirname, 'src/'),
    }
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader',
      },
      {
        test: /\.js$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env']
          }
        }
      },
      {
        test: /\.s[ac]ss$/i,
        use: [
          'vue-style-loader',
          'css-loader?-url', // dont load url
          'sass-loader'
        ]
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
  ]
};
// @todo: https://github.com/samteb/vue-2-webpack-4-boilerplate
