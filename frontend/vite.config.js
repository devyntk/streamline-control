import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
import tailwindNesting from "tailwindcss/nesting";
import tailwindcss from "tailwindcss";

export default defineConfig(({ command, mode, ssrBuild }) => {
  return {
    plugins: [react()],
    server: {
      proxy: {
        "/api": "http://localhost:8080",
      },
    },
    resolve: {
      alias: {
        "~": path.resolve(__dirname, "src"),
      },
    },
    css: {
      postcss: {
        plugins: [tailwindNesting, tailwindcss],
      },
    },
  };
});
