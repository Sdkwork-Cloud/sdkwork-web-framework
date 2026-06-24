import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { defineConfig, loadEnv } from 'vite';
import react from "@vitejs/plugin-react";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, path.dirname(fileURLToPath(import.meta.url)), '');
  const e2eAdminUrl =
    process.env.SDKWORK_E2E_ADMIN_URL?.trim() ||
    env.SDKWORK_E2E_ADMIN_URL?.trim() ||
    'http://127.0.0.1:3920';
  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(
        process.env.SDKWORK_ACCESS_TOKEN ?? env.SDKWORK_ACCESS_TOKEN ?? '',
      ),
    },
          plugins: [react()],
  server: {
    port: 5175,
    proxy: {
      "/backend": {
        target: "http://127.0.0.1:3920",
        changeOrigin: true,
      },
    },
  },
  preview: {
    proxy: {
      "/backend": {
        target: e2eAdminUrl,
        changeOrigin: true,
      },
    },
  },
  };
});