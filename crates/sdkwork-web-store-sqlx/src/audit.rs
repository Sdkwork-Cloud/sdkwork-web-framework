use crate::now_epoch_secs;
use async_trait::async_trait;
use sdkwork_web_core::{AuditEmitter, AuditFact, WebFrameworkError};
use sqlx::SqlitePool;

pub struct SqlxAuditEmitter {
    pool: SqlitePool,
}

impl SqlxAuditEmitter {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AuditEmitter for SqlxAuditEmitter {
    async fn emit(&self, fact: AuditFact) -> Result<(), WebFrameworkError> {
        sqlx::query(
            "INSERT INTO web_audit_event \
             (request_id, tenant_id, user_id, api_surface, path, method, operation_id, status_code, duration_ms, created_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&fact.request_id)
        .bind(&fact.tenant_id)
        .bind(&fact.user_id)
        .bind(format!("{:?}", fact.api_surface))
        .bind(&fact.path)
        .bind(&fact.method)
        .bind(&fact.operation_id)
        .bind(fact.status_code.map(i64::from))
        .bind(fact.duration_ms.map(|value| value as i64))
        .bind(now_epoch_secs())
        .execute(&self.pool)
        .await
        .map_err(|error| {
            WebFrameworkError::dependency_unavailable(format!("sqlx audit store error: {error}"))
        })?;
        Ok(())
    }
}
