use crate::fallback::{contract_fallback_handler, ContractFallbackConfig};
use crate::health::{
    healthz_handler, livez_handler, readyz_handler, AlwaysReady, CompositeReadinessCheck,
    ReadinessCheck,
};
use crate::observability::{metrics_handler, HttpMetricsRegistry};
#[cfg(feature = "redis")]
use crate::redis_readiness::RedisReadinessCheck;
use axum::extract::Request;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct ServiceRouterConfig {
    pub readiness: Option<Arc<dyn ReadinessCheck>>,
    pub metrics: Option<Arc<HttpMetricsRegistry>>,
    pub contract_fallback: Option<ContractFallbackConfig>,
}

impl ServiceRouterConfig {
    pub fn with_always_ready(mut self) -> Self {
        self.readiness = Some(Arc::new(AlwaysReady));
        self
    }

    #[cfg(feature = "sqlx")]
    pub fn with_sqlite_readiness(mut self, pool: sqlx::SqlitePool) -> Self {
        self.readiness = Some(Arc::new(crate::sqlx_readiness::SqliteReadinessCheck::new(
            pool,
        )));
        self
    }

    pub fn with_metrics(mut self, metrics: Arc<HttpMetricsRegistry>) -> Self {
        self.metrics = Some(metrics);
        self
    }

    pub fn with_readiness_check(mut self, check: Arc<dyn ReadinessCheck>) -> Self {
        self.readiness = Some(check);
        self
    }

    pub fn with_contract_fallback(mut self, config: ContractFallbackConfig) -> Self {
        self.contract_fallback = Some(config);
        self
    }

    pub fn with_contract_fallback_from_manifest(
        mut self,
        manifest: &sdkwork_web_core::HttpRouteManifest,
    ) -> Self {
        self.contract_fallback = Some(ContractFallbackConfig::from_manifest(manifest));
        self
    }

    pub fn with_composite_readiness(mut self, checks: Vec<Arc<dyn ReadinessCheck>>) -> Self {
        self.readiness = Some(Arc::new(CompositeReadinessCheck::new(checks)));
        self
    }

    #[cfg(feature = "redis")]
    pub fn with_redis_readiness(
        mut self,
        redis_url: impl AsRef<str>,
    ) -> Result<Self, redis::RedisError> {
        self.readiness = Some(Arc::new(RedisReadinessCheck::new(redis_url)?));
        Ok(self)
    }

    pub fn metrics(&self) -> Option<Arc<HttpMetricsRegistry>> {
        self.metrics.clone()
    }
}

/// Mounts `/healthz`, `/readyz`, `/metrics`, and optional contract fallback on the supplied router.
pub fn service_router(router: Router, config: ServiceRouterConfig) -> Router {
    mount_infra_routes(router, config)
}

/// Same as [`service_router`] but preserves router state type `S` (for `Router<AppState>` processes).
pub fn mount_infra_routes<S>(router: Router<S>, config: ServiceRouterConfig) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let metrics = match config.metrics {
        Some(metrics) => metrics,
        None => HttpMetricsRegistry::new(),
    };
    let readiness = config.readiness;
    let mut router = router
        .route("/healthz", get(healthz_handler))
        .route("/livez", get(livez_handler))
        .route(
            "/readyz",
            get(move || {
                let readiness = readiness.clone();
                async move { readyz_handler(readiness).await }
            }),
        )
        .route(
            "/metrics",
            get({
                let metrics = metrics.clone();
                move || {
                    let metrics = metrics.clone();
                    async move { metrics_handler(metrics).await }
                }
            }),
        );
    if let Some(fallback_config) = config.contract_fallback {
        router = router.fallback(move |request: Request| {
            let config = fallback_config.clone();
            async move { contract_fallback_handler(request, config).await }
        });
    }
    router
}
