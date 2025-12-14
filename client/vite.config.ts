import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	build: {
	  rollupOptions: {
			  output: {
					cssCodeSplits: false,
			    manualChunks: (id) => {
            // Chunk 1: All node_modules dependencies into 'vendor'
            if (id.includes('node_modules')) {
              return 'vendor';
            }

            // Chunk 2: Your app code stays in default chunk
            // Everything else goes to the main app chunk automatically
          }
				}
			}
	},
	server: {
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        secure: false,
        // Optional: rewrite the path if needed
        // rewrite: (path) => path.replace(/^\/api/, '')
      },
      '/pokemon': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        secure: false,
        // Optional: rewrite the path if needed
        // rewrite: (path) => path.replace(/^\/api/, '')
      }
    }
  }
});
