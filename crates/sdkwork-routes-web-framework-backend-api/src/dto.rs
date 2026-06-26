use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsPolicyRecord {
    pub tenant_id: String,
    pub environment: String,
    pub allow_all_origins: bool,
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertCorsPolicyRequest {
    pub tenant_id: String,
    pub environment: String,
    pub allow_all_origins: bool,
    pub allowed_origins: Vec<String>,
    pub allow_credentials: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitPolicyRecord {
    pub tenant_id: String,
    pub environment: String,
    pub tier_key: String,
    pub max_requests: u32,
    pub window_secs: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertRateLimitPolicyRequest {
    pub tenant_id: String,
    pub environment: String,
    pub tier_key: String,
    pub max_requests: u32,
    pub window_secs: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantRuntimeProfileRecord {
    pub tenant_id: String,
    pub environment: String,
    pub rate_limit_enabled: Option<bool>,
    pub max_content_length: Option<u64>,
    pub max_concurrent_requests: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpsertTenantRuntimeProfileRequest {
    pub tenant_id: String,
    pub environment: String,
    pub rate_limit_enabled: Option<bool>,
    pub max_content_length: Option<u64>,
    pub max_concurrent_requests: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEventRecord {
    pub id: i64,
    pub kind: String,
    pub request_id: Option<String>,
    pub path: String,
    pub method: String,
    pub api_surface: String,
    pub origin: Option<String>,
    pub detail: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventRecord {
    pub id: i64,
    pub request_id: String,
    pub tenant_id: Option<String>,
    pub user_id: Option<String>,
    pub api_surface: String,
    pub path: String,
    pub method: String,
    pub operation_id: Option<String>,
    pub status_code: Option<i64>,
    pub duration_ms: Option<i64>,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlNodeRecord {
    pub node_id: String,
    pub region: String,
    pub base_url: String,
    pub environment: String,
    pub status: String,
    pub last_heartbeat_at: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RegisterControlNodeOutcome {
    pub record: ControlNodeRecord,
    pub created: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterControlNodeRequest {
    pub node_id: String,
    pub region: Option<String>,
    pub base_url: String,
    pub environment: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RuntimeDefaultsSnapshot {
    pub production_security_policy: serde_json::Value,
    pub default_security_policy: serde_json::Value,
    pub optional_features_production_sqlx: sdkwork_web_core::WebFrameworkOptionalFeatures,
}

#[derive(Debug, Clone, Serialize)]
pub struct OptionalFeaturesSnapshot {
    pub recommended_production_sqlx: sdkwork_web_core::WebFrameworkOptionalFeatures,
    pub development: sdkwork_web_core::WebFrameworkOptionalFeatures,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListQuery {
    pub environment: Option<String>,
    pub tenant_id: Option<String>,
    pub limit: Option<u32>,
}
