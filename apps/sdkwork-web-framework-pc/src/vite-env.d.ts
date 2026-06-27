/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_SDKWORK_WEB_FRAMEWORK_BACKEND_API_BASE_URL?: string;
  /** Dev/E2E-only baked Access-Token JWT. NEVER set in production builds. */
  readonly VITE_SDKWORK_ACCESS_TOKEN?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
