import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import graphql from 'vite-plugin-simple-gql'

const replaceIndexToDjangoBase = () => {
  return {
    name: 'renameIndex',
    enforce: 'post',
    generateBundle(options, bundle) {
      const indexHtml = bundle['index.html']
      indexHtml.fileName = "../web/templates/web/base_vue.html"
    }
  };
};

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue(), vueJsx(),  replaceIndexToDjangoBase()],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  },
  build: {
    emptyOutDir: true,
    outDir: "../web/static",
  },
})
