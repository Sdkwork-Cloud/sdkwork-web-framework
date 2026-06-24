use async_trait::async_trait;
use sdkwork_web_core::{
    rate_limit::ResolvedRateLimitPolicy,
    rate_limit_policy::{DynamicRateLimitPolicySource, RateLimitPolicyContext},
    WebFrameworkError,
};
use sqlx::SqlitePool;

use crate::purge::sqlx_error;

pub struct SqlxRateLimitPolicySource {
    pool: SqlitePool,
}

impl SqlxRateLimitPolicySource {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    async fn lookup_row(
        &self,
        tenant_id: &str,
        environment: &str,
        tier_key: &str,
    ) -> Result<Option<RateLimitPolicyRow>, WebFrameworkError> {
        sqlx::query_as::<_, RateLimitPolicyRow>(
            "SELECT max_requests, window_secs, enabled FROM web_rate_limit_policy \
             WHERE tenant_id = ? AND environment = ? AND tier_key = ?",
        )
        .bind(tenant_id)
        .bind(environment)
        .bind(tier_key)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error)
    }
}

#[derive(sqlx::FromRow)]
struct RateLimitPolicyRow {
    max_requests: i64,
    window_secs: i64,
    enabled: i64,
}

#[async_trait]
impl DynamicRateLimitPolicySource for SqlxRateLimitPolicySource {
    async fn resolve(
        &self,
        ctx: &RateLimitPolicyContext,
    ) -> Result<Option<ResolvedRateLimitPolicy>, WebFrameworkError> {
        let environment = ctx.environment_label();
        let tier_key = ctx.tier_key();
        let tenant = ctx.tenant_scope();

        let candidates = [
            (tenant, tier_key),
            (tenant, "default"),
            ("0", tier_key),
            ("0", "default"),
        ];

        for (tenant_id, tier) in candidates {
            let Some(row) = self.lookup_row(tenant_id, environment, tier).await? else {
                continue;
            };
            if row.enabled == 0 {
                return Ok(Some(ResolvedRateLimitPolicy {
                    max_requests: 0,
                    window_secs: row.window_secs.max(1) as u64,
                }));
            }
            return Ok(Some(ResolvedRateLimitPolicy {
                max_requests: row.max_requests.max(0) as u32,
                window_secs: row.window_secs.max(1) as u64,
            }));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use sdkwork_web_core::{
        rate_limit_policy::{rate_limit_tier_key, RateLimitPolicyContext},
        RateLimitTier, WebApiSurface, WebEnvironment,
    };

    #[test]
    fn tier_key_matches_manifest_tiers() {
        assert_eq!(
            "auth_critical",
            rate_limit_tier_key(Some(RateLimitTier::AuthCritical))
        );
    }

    #[test]
    fn context_tier_key_defaults_to_default() {
        let ctx = RateLimitPolicyContext {
            tenant_id: None,
            environment: WebEnvironment::Prod,
            api_surface: WebApiSurface::AppApi,
            rate_limit_tier: None,
            operation_id: None,
        };
        assert_eq!("default", ctx.tier_key());
    }
}
