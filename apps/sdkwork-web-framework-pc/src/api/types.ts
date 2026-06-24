export type ApiEnvelope<T> = { success: boolean; data: T; message?: string };

export type CorsPolicyRecord = {
  tenant_id: string;
  environment: string;
  allow_all_origins: boolean;
  allowed_origins: string[];
  allow_credentials: boolean;
};

export type RateLimitPolicyRecord = {
  tenant_id: string;
  environment: string;
  tier_key: string;
  max_requests: number;
  window_secs: number;
  enabled: boolean;
};

export type TenantRuntimeProfileRecord = {
  tenant_id: string;
  environment: string;
  rate_limit_enabled?: boolean | null;
  max_content_length?: number | null;
  max_concurrent_requests?: number | null;
};

export type ControlNodeRecord = {
  node_id: string;
  region: string;
  base_url: string;
  environment: string;
  status: string;
  last_heartbeat_at?: number | null;
  created_at: number;
  updated_at: number;
};

export type RuntimeDefaultsSnapshot = {
  production_security_policy: Record<string, unknown>;
  default_security_policy: Record<string, unknown>;
  optional_features_production_sqlx: Record<string, boolean>;
};

export type OptionalFeaturesSnapshot = {
  recommended_production_sqlx: Record<string, boolean>;
  development: Record<string, boolean>;
};

export type SecurityEventRecord = {
  id: number;
  kind: string;
  request_id?: string | null;
  path: string;
  method: string;
  api_surface: string;
  origin?: string | null;
  detail: string;
  created_at: number;
};

export type AuditEventRecord = {
  id: number;
  request_id: string;
  tenant_id?: string | null;
  user_id?: string | null;
  api_surface: string;
  path: string;
  method: string;
  operation_id?: string | null;
  status_code?: number | null;
  duration_ms?: number | null;
  created_at: number;
};
