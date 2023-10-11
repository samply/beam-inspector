import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  build: {
    emptyOutDir: true,
  },
  server: {
    proxy: {
        '/events': 'http://localhost:1337',
        '/mappings': 'http://localhost:1337',
    }
  }
})
