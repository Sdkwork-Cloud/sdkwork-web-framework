import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { defineConfig, loadEnv } from 'vite';
import react from "@vitejs/plugin-react";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, path.dirname(fileURLToPath(import.meta.url)), '');
  // Dev/preview backend target. `SDKWORK_E2E_ADMIN_URL` is the integration-E2E override;
  // `SDKWORK_WEB_FRAMEWORK_ADMIN_URL` is the local dev override. Both default to the admin server.
  const backendUrl =
    process.env.SDKWORK_E2E_ADMIN_URL?.trim() ||
    env.SDKWORK_E2E_ADMIN_URL?.trim() ||
    env.SDKWORK_WEB_FRAMEWORK_ADMIN_URL?.trim() ||
    'http://127.0.0.1:3920';
  return {
    plugins: [react()],
    server: {
      port: 5175,
      proxy: {
        "/backend": {
          target: backendUrl,
          changeOrigin: true,
        },
      },
    },
    preview: {
      proxy: {
        "/backend": {
          target: backendUrl,
          changeOrigin: true,
        },
      },
    },
  };
});
