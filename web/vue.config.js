const { defineConfig } = require("@vue/cli-service");
module.exports = defineConfig({
  chainWebpack: config => {
    config
      .plugin('html')
      .tap(args => {
        args[0].title = "Anthill";
        return args;
      })
  },
  transpileDependencies: ["vuetify"],
  devServer: {
    proxy: {
      "^/api/": {
        target: "http://localhost:8081",
      },
    },
  },
});
