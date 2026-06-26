//! Canonical path constants for framework control-plane backend-api (`WEB_BACKEND_SPEC.md`).

macro_rules! framework_path {
    ($suffix:literal) => {
        concat!("/backend/v3/api/web-framework", $suffix)
    };
}

pub const API_PREFIX: &str = "/backend/v3/api/web-framework";

pub mod cors {
    pub const PATH: &str = framework_path!("/cors-policies");
}

pub mod rate_limit {
    pub const PATH: &str = framework_path!("/rate-limit-policies");
}

pub mod tenant_runtime {
    pub const PATH: &str = framework_path!("/tenant-runtime-profiles");
}

pub mod security_events {
    pub const PATH: &str = framework_path!("/security-events");
}

pub mod audit_events {
    pub const PATH: &str = framework_path!("/audit-events");
}

pub mod control_nodes {
    pub const COLLECTION: &str = framework_path!("/control-nodes");
    pub const BY_ID: &str = framework_path!("/control-nodes/{node_id}");
    pub const HEARTBEAT: &str = framework_path!("/control-nodes/{node_id}/heartbeat");
}

pub mod runtime_defaults {
    pub const PATH: &str = framework_path!("/runtime-defaults");
}

pub mod optional_features {
    pub const PATH: &str = framework_path!("/optional-features");
}
