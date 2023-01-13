import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig(({ command, mode, ssrBuild }) => {
  return {
    plugins: [react()],
    server: {
      proxy: {
        "/api": "http://localhost:8080",
      },
    },
  };
});
