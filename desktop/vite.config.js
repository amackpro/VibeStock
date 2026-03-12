import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  // Tauri uses a specific port in dev mode
  server: {
    port: 1420,
    strictPort: true,
    host: '127.0.0.1',
  },
  // Prevent asset path issues in Tauri
  base: './',
  build: {
    outDir: 'dist',
    emptyOutDir: true,
  }
})
