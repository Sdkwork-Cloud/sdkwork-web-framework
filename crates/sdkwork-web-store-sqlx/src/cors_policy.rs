use async_trait::async_trait;
use sdkwork_web_core::{
    cors_policy::{CorsPolicyContext, DynamicCorsPolicySource},
    CorsPolicy, WebFrameworkError,
};
use sqlx::SqlitePool;

use crate::purge::sqlx_error;

pub struct SqlxCorsPolicySource {
    pool: SqlitePool,
}

impl SqlxCorsPolicySource {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct CorsPolicyRow {
    allow_all_origins: i64,
    allowed_origins: String,
    allow_credentials: i64,
}

#[async_trait]
impl DynamicCorsPolicySource for SqlxCorsPolicySource {
    async fn resolve(
        &self,
        ctx: &CorsPolicyContext,
    ) -> Result<Option<CorsPolicy>, WebFrameworkError> {
        let row = sqlx::query_as::<_, CorsPolicyRow>(
            "SELECT allow_all_origins, allowed_origins, allow_credentials \
             FROM web_cors_policy WHERE tenant_id = ? AND environment = ?",
        )
        .bind(ctx.tenant_scope())
        .bind(ctx.environment_label())
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error)?;

        let Some(row) = row else {
            return Ok(None);
        };

        let allowed_origins: Vec<String> =
            serde_json::from_str(&row.allowed_origins).map_err(|error| {
                WebFrameworkError::dependency_unavailable(format!(
                    "sqlx cors policy decode error: {error}"
                ))
            })?;

        Ok(Some(CorsPolicy {
            allow_all_origins: row.allow_all_origins != 0,
            allowed_origins,
            allowed_methods: CorsPolicy::default().allowed_methods,
            allowed_headers: CorsPolicy::default().allowed_headers,
            allow_credentials: row.allow_credentials != 0,
        }))
    }
}
