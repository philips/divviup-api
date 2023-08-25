import { defineConfig } from "vite";
import eslint from "vite-plugin-eslint";
import react from "@vitejs/plugin-react-swc";

export default defineConfig(() => ({
  build: {
    sourcemap: true,
    outDir: process.env.BUILD_PATH || "build",
  },
  plugins: [react(), eslint()],
  server: {
    port: 8081,
    strictPort: true,
  },
}));
