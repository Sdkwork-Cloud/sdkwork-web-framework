use crate::now_epoch_secs;
use async_trait::async_trait;
use sdkwork_web_core::{SecurityEvent, SecurityEventEmitter, SecurityEventKind, WebFrameworkError};
use sqlx::SqlitePool;

pub struct SqlxSecurityEventEmitter {
    pool: SqlitePool,
}

impl SqlxSecurityEventEmitter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SecurityEventEmitter for SqlxSecurityEventEmitter {
    async fn emit(&self, event: SecurityEvent) -> Result<(), WebFrameworkError> {
        sqlx::query(
            "INSERT INTO web_security_event
             (kind, request_id, path, method, api_surface, origin, detail, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(security_event_kind_label(&event.kind))
        .bind(event.request_id)
        .bind(event.path)
        .bind(event.method)
        .bind(format!("{:?}", event.api_surface))
        .bind(event.origin)
        .bind(event.detail)
        .bind(now_epoch_secs())
        .execute(&self.pool)
        .await
        .map_err(|error| {
            WebFrameworkError::dependency_unavailable(format!(
                "sqlx security event store error: {error}"
            ))
        })?;
        Ok(())
    }
}

fn security_event_kind_label(kind: &SecurityEventKind) -> &'static str {
    match kind {
        SecurityEventKind::CorsDenied => "cors_denied",
        SecurityEventKind::RateLimitExceeded => "rate_limit_exceeded",
        SecurityEventKind::AuthenticationFailed => "authentication_failed",
        SecurityEventKind::AuthorizationDenied => "authorization_denied",
        SecurityEventKind::TenantIsolationDenied => "tenant_isolation_denied",
    }
}
