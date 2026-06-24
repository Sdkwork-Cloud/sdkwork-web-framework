use async_trait::async_trait;
use sdkwork_web_core::{
    tenant_runtime::{
        DynamicTenantRuntimeProfileSource, TenantRuntimeProfile, TenantRuntimeProfileContext,
    },
    WebFrameworkError,
};
use sqlx::SqlitePool;

use crate::purge::sqlx_error;

pub struct SqlxTenantRuntimeProfileSource {
    pool: SqlitePool,
}

impl SqlxTenantRuntimeProfileSource {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct TenantRuntimeProfileRow {
    rate_limit_enabled: Option<i64>,
    max_content_length: Option<i64>,
    max_concurrent_requests: Option<i64>,
}

#[async_trait]
impl DynamicTenantRuntimeProfileSource for SqlxTenantRuntimeProfileSource {
    async fn resolve(
        &self,
        ctx: &TenantRuntimeProfileContext,
    ) -> Result<Option<TenantRuntimeProfile>, WebFrameworkError> {
        let row = sqlx::query_as::<_, TenantRuntimeProfileRow>(
            "SELECT rate_limit_enabled, max_content_length, max_concurrent_requests \
             FROM web_tenant_runtime_profile WHERE tenant_id = ? AND environment = ?",
        )
        .bind(ctx.tenant_scope())
        .bind(ctx.environment_label())
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error)?;

        let Some(row) = row else {
            return Ok(None);
        };

        Ok(Some(TenantRuntimeProfile {
            rate_limit_enabled: row.rate_limit_enabled.map(|value| value != 0),
            max_content_length: row
                .max_content_length
                .and_then(|value| u64::try_from(value.max(0)).ok()),
            max_concurrent_requests: row
                .max_concurrent_requests
                .and_then(|value| u32::try_from(value.max(0)).ok()),
        }))
    }
}
