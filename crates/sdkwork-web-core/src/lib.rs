//! SDKWork web framework core: request context, interceptor pipeline, security policies.

pub mod api_chain;
pub mod axum_integration;
pub mod client_context_guard;
pub mod client_kind;
pub mod constants;
pub mod context_injection;
pub mod cors_policy;
pub mod error;
pub mod extractors;
pub mod hashing;
pub mod idempotency;
pub mod interceptors;
pub mod jwt;
pub mod jwt_claims;
pub mod jwt_fixtures;
pub mod jwt_tenant;
pub mod metrics;
pub mod open_api_auth;
pub mod parsers;
pub mod path_resource_guard;
pub mod policies;
pub mod policy_cache;
pub mod problem;
pub mod production_assembly;
pub mod rate_limit;
pub mod rate_limit_policy;
pub mod redact;
pub mod request_context;
pub mod request_identity;
pub mod resolvers;
pub mod route_manifest;
pub mod runtime_options;
pub mod security;
pub mod stores;
pub mod surface;
pub mod surface_bridge;
pub mod tenant_app_context;
pub mod tenant_runtime;
pub mod token_version;
pub mod trace;
pub mod websocket;
pub mod ws_interceptors;

#[cfg(test)]
mod pipeline_contract_tests;

pub use api_chain::{
    WebCallCredentials, WebCallCredentials as ApiCallCredentials, WebCallInterceptor,
    WebCallInterceptor as ApiCallInterceptor, WebCallInterceptorChain,
    WebCallInterceptorChain as ApiCallInterceptorChain, WebCallRuntime,
    WebCallRuntime as ApiCallRuntime, WebCallStage, WebCallStage as ApiCallStage, WebCallState,
    WebCallState as ApiCallState, WebFrameworkRuntime, STANDARD_STAGE_ORDER,
};
pub use axum_integration::{
    RequireAppApi, RequireDualToken, RequireOpenApi, RequirePrincipal, RequireTenantApp,
};
pub use client_context_guard::{
    inspect_json_body_context_selectors, is_forbidden_context_selector_key,
    reject_client_context_selectors, reject_forbidden_ambient_context_path,
    reject_forbidden_context_body_json, reject_forbidden_context_query,
    requires_client_context_selector_guard,
};
pub use constants::{
    ACCESS_TOKEN_HEADER, API_KEY_HEADER, APP_API_PREFIX, AUTHORIZATION_HEADER, BACKEND_API_PREFIX,
    CONTENT_SHA256_HEADER, DYNAMIC_POLICY_CACHE_TTL_SECS, GATEWAY_API_PREFIX,
    IDEMPOTENCY_FINGERPRINT_HEADER, IDEMPOTENCY_KEY_HEADER, OPEN_API_PREFIX, OPERATION_ID_HEADER,
    PRODUCTION_DEFAULT_REQUEST_TIMEOUT_SECS, PRODUCTION_DEFAULT_SHUTDOWN_GRACE_SECS,
    REQUEST_ID_HEADER, WS_APP_API_SUFFIX, X_IDEMPOTENCY_KEY_HEADER,
};
pub use context_injection::{inject_web_request_context, DomainContextInjector};
pub use cors_policy::{CorsPolicyContext, DynamicCorsPolicySource, NoOpDynamicCorsPolicySource};
pub use error::{
    AppRequestContextError, AppRequestContextErrorKind, WebFrameworkError, WebFrameworkErrorKind,
    WebFrameworkRejection,
};
pub use idempotency::{
    content_length_from_headers, idempotency_fingerprint, idempotency_replay_response,
    resolve_idempotency_fingerprint, IdempotencyBeginOutcome, IdempotencyResponseRecord,
};
pub use interceptors::{
    StandardWebCallInterceptor, StandardWebCallInterceptor as StandardApiCallInterceptor,
    StandardWebCallInterceptorKind,
    StandardWebCallInterceptorKind as StandardApiCallInterceptorKind,
};
pub use jwt::{
    is_hmac_sha256_jwt_verifier, is_payload_only_jwt_verifier, uses_global_shared_jwt_verifier,
    validate_production_jwt_verifier, HmacSha256JwtVerifier, JwtVerifier, PayloadOnlyJwtVerifier,
    StrictJwtVerifier, VerifyingAccessTokenParser, VerifyingAuthTokenParser,
};
pub use jwt_claims::{validate_jwt_token_type_claim, JwtProductionClaimPolicy};
pub use jwt_fixtures::{
    access_token_jwt, auth_token_jwt, auth_token_jwt_with_permissions, bootstrap_access_token_jwt,
    encode_hs256_test_jwt, encode_hs256_test_jwt_with_kid, encode_hs256_test_jwt_without_kid,
    encode_rs256_test_jwt_with_kid, encode_unsigned_test_jwt, generate_rs256_test_keypair,
};
pub use jwt_tenant::{
    EnvBootstrapTenantSigningKeyLookup, JwtSessionRevocationChecker,
    NoOpJwtSessionRevocationChecker, StaticJwtSessionRevocationChecker,
    StaticTenantSigningKeyLookup, TenantBoundJwtVerifier, TenantSigningKeyAlgorithm,
    TenantSigningKeyLookup, TenantSigningKeyMaterial,
};
pub use metrics::{
    environment_metric_label, http_request_labels_from_state, HttpMetricsDimensions,
    HttpMetricsRegistry, HttpRequestLabels,
};
pub use open_api_auth::{
    allowed_open_api_schemes, default_open_api_scheme_detector, resolve_open_api_request_context,
    DefaultOpenApiCredentialSchemeDetector, DynOpenApiCredentialSchemeDetector, OpenApiAuthPolicy,
    OpenApiAuthScheme, OpenApiCredentialSchemeDetector,
};
pub use parsers::{
    AccessTokenClaims, AccessTokenParser, ApiKeyCredential, ApiKeyParser, AuthTokenClaims,
    AuthTokenParser, DefaultAccessTokenParser, DefaultApiKeyParser, DefaultAuthTokenParser,
    DefaultOAuthBearerParser, OAuthBearerCredential, OAuthBearerParser,
};
pub use path_resource_guard::{
    extract_path_resource_ids, verify_path_resource_ids_match_principal, PathResourceIds,
};
pub use policies::{
    AllowAllAuthorizationPolicy, AuditEmitter, AuditFact, AuthorizationPolicy,
    DenyAllAuthorizationPolicy, EnforcePrincipalTenantIsolationPolicy, ManifestAuthorizationPolicy,
    NoOpAuditEmitter, NoOpSecurityEventEmitter, PassThroughTenantIsolationPolicy, SecurityEvent,
    SecurityEventEmitter, SecurityEventKind, TenantIsolationPolicy,
};
pub use policy_cache::{
    CachingDynamicCorsPolicySource, CachingDynamicRateLimitPolicySource,
    CachingDynamicTenantRuntimeProfileSource, DynamicPolicyCaches, TtlCache,
};
pub use problem::{problem_response, redact_path_template, ProblemCorrelation};
pub use production_assembly::{
    requires_production_assembly, validate_deployment_environment, validate_production_assembly,
    ProductionAssemblyInput,
};
pub use rate_limit::{
    limits_for_tier, DefaultRateLimitPolicyResolver, RateLimitPolicyResolver,
    ResolvedRateLimitPolicy,
};
pub use rate_limit_policy::{
    rate_limit_tier_key, DynamicRateLimitPolicySource, NoOpDynamicRateLimitPolicySource,
    RateLimitPolicyContext,
};
pub use redact::{is_redacted_log_field, redact_sensitive_header, redact_sensitive_log_value};
pub use request_context::{
    AppRequestApiSurface, AppRequestAuthLevel, AppRequestAuthMode, AppRequestContext,
    AppRequestContextProfile, AppRequestDeploymentMode, AppRequestEnvironment,
    AppRequestLoginScope, AppRequestPrincipal, WebApiSurface, WebAppContext, WebAuthContext,
    WebAuthLevel, WebAuthMode, WebClientKind, WebDeploymentMode, WebEnvironment, WebLoginScope,
    WebOperationBinding, WebRequestContext, WebRequestContextProfile, WebRequestPrincipal,
    WebRequestPrincipalBuilder, WebScopeContext, WebSubjectContext, WebSubjectType,
    WebTenancyContext, WebTransportFacts,
};
pub use request_identity::{
    is_canonical_uuid, new_request_id, resolve_request_id, ServerRequestId,
};
pub use resolvers::{
    is_tenant_bound_verifying_resolver, tenant_bound_saas_verifying_web_request_resolver,
    tenant_bound_saas_verifying_web_request_resolver_with_claim_policy,
    tenant_bound_verifying_web_request_resolver,
    tenant_bound_verifying_web_request_resolver_with_claim_policy,
    verifying_open_api_web_request_resolver, verifying_web_request_resolver, ApiKeyLookupService,
    ApiKeyPrincipalRecord, DefaultApiKeyLookupService, DefaultOAuthTokenLookupService,
    DefaultOpenApiWebRequestContextResolver, DefaultWebRequestContextResolver,
    DefaultWebRequestContextResolver as DefaultAppRequestContextResolver,
    DisabledApiKeyLookupService, OAuthPrincipalRecord, OAuthTokenLookupService,
    OpenApiWebRequestParserResolver, ResolverProductionProfile,
    TenantBoundProductionWebRequestResolver, VerifyingOAuthBearerParser, WebRequestContextResolver,
    WebRequestContextResolver as AppRequestContextResolver, WebRequestParserResolver,
    WebRequestParserResolver as AppRequestParserResolver,
};
pub use route_manifest::{route_path_matches, HttpRouteManifest};
pub use runtime_options::WebFrameworkOptionalFeatures;
pub use sdkwork_web_contract::{
    ApiSurface, HttpMethod, HttpRoute, IamHttpRoute, RateLimitTier, RouteAuth,
};
pub use security::{
    CorsPolicy, CrossSiteRequestPolicy, HeaderSecurityPolicy, IdempotencyPolicy,
    JsonContentTypePolicy, MethodGuardPolicy, RateLimitPolicy, RequestSecurityPolicy,
    RequestSizeLimitPolicy, SecurityPolicy, SqlInjectionGuardPolicy,
};
pub use stores::{
    memory_concurrent_admission_store, memory_idempotency_store, memory_rate_limit_store,
    ConcurrentAdmissionStore, IdempotencyStore, MemoryConcurrentAdmissionStore,
    MemoryIdempotencyStore, MemoryRateLimitStore, RateLimitStore,
};
pub use surface::{
    api_surface_contract_label, api_surface_metric_label, classify_api_surface, resolve_public_path,
};
pub use tenant_app_context::TenantAppContext;
pub use tenant_runtime::{
    DynamicTenantRuntimeProfileSource, NoOpDynamicTenantRuntimeProfileSource, TenantRuntimeProfile,
    TenantRuntimeProfileContext,
};
pub use token_version::{
    extract_token_version_from_json, stamp_token_version, validate_token_version,
    validate_token_version_claims, validate_token_version_json, TokenVersionPolicy,
    SDKWORK_TOKEN_VERSION_CURRENT,
};
pub use trace::{
    resolve_trace_context, trace_id_from_traceparent, TraceContext, TRACEPARENT_HEADER,
    TRACESTATE_HEADER,
};
pub use websocket::{
    WebSocketCallInterceptor, WebSocketCallInterceptorChain, WebSocketCallRuntime,
    WebSocketCallStage, WebSocketCallState, WebSocketMessageFrame, WebSocketSession,
};
pub use ws_interceptors::{StandardWebSocketCallInterceptor, StandardWebSocketCallInterceptorKind};
#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::collections::BTreeMap;

    #[test]
    fn classifies_standard_api_surfaces() {
        let profile = WebRequestContextProfile::default();
        assert_eq!(
            WebApiSurface::AppApi,
            classify_api_surface("/app/v3/api/auth/sessions", &profile)
        );
        assert_eq!(
            WebApiSurface::BackendApi,
            classify_api_surface("/backend/v3/api/iam/users", &profile)
        );
        assert_eq!(
            WebApiSurface::OpenApi,
            classify_api_surface("/open/v3/api/messages", &profile)
        );
    }

    #[test]
    fn chain_order_matches_api_spec() {
        let chain = WebCallInterceptorChain::<DefaultWebRequestContextResolver>::standard();
        assert_eq!(18, chain.interceptor_count());
        assert_eq!(
            STANDARD_STAGE_ORDER.as_slice(),
            chain.stage_order().as_slice()
        );
    }

    #[test]
    fn web_request_context_json_matches_schema_vocabulary() {
        use crate::request_context::{WebClientKind, WebOperationBinding};
        use sdkwork_web_contract::RateLimitTier;

        let ctx = WebRequestContext {
            request_id: ServerRequestId("550e8400-e29b-41d4-a716-446655440000".to_owned()),
            api_surface: WebApiSurface::BackendApi,
            auth_mode: WebAuthMode::DualToken,
            principal: None,
            transport: WebTransportFacts {
                path: "/backend/v3/api/web-framework/cors-policies".to_owned(),
                method: "GET".to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
            },
            locale: Some("en-US".to_owned()),
            client_kind: Some(WebClientKind::Browser),
            operation: Some(WebOperationBinding {
                operation_id: "webFramework.corsPolicies.list".to_owned(),
                route_template: "/backend/v3/api/web-framework/cors-policies".to_owned(),
                rate_limit_tier: Some(RateLimitTier::OpenApiDefault),
                idempotent: false,
            }),
            trace_id: None,
        };
        let json = serde_json::to_value(&ctx).expect("serialize context");
        assert!(json.get("requestId").is_some());
        assert!(json.get("apiSurface").is_some());
        assert!(json.get("authMode").is_some());
        assert!(json.get("transport").is_some());
        let transport = json["transport"].as_object().expect("transport object");
        assert!(transport.contains_key("authTokenPresent"));
        assert!(transport.contains_key("accessTokenPresent"));
        assert!(transport.contains_key("apiKeyPresent"));
        assert!(transport.contains_key("oauthBearerPresent"));
        assert!(json.get("clientKind").is_some());
        assert!(json.get("operation").is_some());
    }

    #[test]
    fn api_surface_bridge_matches_contract() {
        assert_eq!(
            WebApiSurface::AppApi,
            WebApiSurface::from(ApiSurface::AppApi)
        );
        assert_eq!(
            ApiSurface::OpenApi,
            ApiSurface::from(WebApiSurface::OpenApi)
        );
    }

    #[test]
    fn workspace_manifest_has_no_business_dependencies() {
        let manifest = include_str!("../Cargo.toml");
        for forbidden in ["sdkwork-appbase", "sdkwork-clawrouter", "openchat"] {
            assert!(
                !manifest.contains(forbidden),
                "core crate must not depend on {forbidden}"
            );
        }
    }

    #[test]
    fn standard_call_chain_declares_common_interceptors() {
        let chain = WebCallInterceptorChain::<DefaultWebRequestContextResolver>::standard();
        assert_eq!(18, chain.interceptor_count());
    }

    #[test]
    fn production_runtime_enables_rate_limit_and_deny_all_auth() {
        let runtime = WebCallRuntime::production(DefaultWebRequestContextResolver::default());
        assert!(runtime.security_policy.rate_limit.enabled);
        let ctx = WebRequestContext {
            request_id: ServerRequestId("req-1".to_owned()),
            api_surface: WebApiSurface::AppApi,
            auth_mode: WebAuthMode::DualToken,
            principal: Some(
                WebRequestPrincipal::builder()
                    .tenant_id("100001")
                    .user_id("user-1")
                    .app_id("appbase")
                    .build(),
            ),
            transport: WebTransportFacts {
                path: "/app/v3/api/users".to_owned(),
                method: "GET".to_owned(),
                auth_token_present: true,
                access_token_present: true,
                api_key_present: false,
                oauth_bearer_present: false,
            },
            locale: None,
            client_kind: None,
            operation: None,
            trace_id: None,
        };
        let error = runtime
            .authorization
            .authorize(&ctx, Some("listUsers"))
            .expect_err("deny all");
        assert_eq!(WebFrameworkErrorKind::Forbidden, error.kind);
    }

    #[tokio::test]
    async fn verifying_resolver_rejects_inline_claim_strings() {
        use crate::jwt::PayloadOnlyJwtVerifier;
        use crate::verifying_web_request_resolver;
        use std::sync::Arc;

        let resolver = verifying_web_request_resolver(
            Arc::new(PayloadOnlyJwtVerifier),
            DefaultApiKeyLookupService,
        );
        let error = resolver
            .resolve_dual_token(
                "tenant_id=100001;user_id=30;session_id=session-1;app_id=appbase;auth_level=password",
                "tenant_id=100001;user_id=30;session_id=session-1;app_id=appbase;environment=prod;deployment_mode=saas",
            )
            .await
            .expect_err("inline claims rejected");
        assert_eq!(WebFrameworkErrorKind::InvalidCredentials, error.kind);
    }

    #[tokio::test]
    async fn default_api_key_resolver_maps_claims_to_principal() {
        let resolver = DefaultWebRequestContextResolver::default();
        let principal = resolver
            .resolve_api_key(
                "api_key_id=key-1;tenant_id=100001;organization_id=0;user_id=30;app_id=appbase;environment=prod;deployment_mode=saas;data_scope=tenant;permission_scope=iam.read,settings.read",
            )
            .await
            .expect("principal");

        assert_eq!("100001", principal.tenant_id());
        assert_eq!(Some("0"), principal.organization_id());
        assert_eq!("30", principal.user_id());
        assert_eq!(Some("key-1"), principal.api_key_id());
        assert_eq!(WebAuthLevel::ApiKey, principal.auth_level());
    }

    #[tokio::test]
    async fn default_access_token_resolver_establishes_tenant_isolation() {
        let resolver = DefaultWebRequestContextResolver::default();
        let principal = resolver
            .resolve_access_token(&bootstrap_access_token_jwt("100001", "app_tenant-bootstrap"))
            .await
            .expect("access token principal");

        assert_eq!("100001", principal.tenant_id());
        assert_eq!("app_tenant-bootstrap", principal.app_id());
        assert_eq!(WebAuthLevel::Anonymous, principal.auth_level());
    }

    #[tokio::test]
    async fn default_dual_token_resolver_rejects_mismatched_tenant() {
        let resolver = DefaultWebRequestContextResolver::default();
        let error = resolver
            .resolve_dual_token(
                &auth_token_jwt("100001", "1", "session-1", "appbase"),
                &access_token_jwt("100002", "1", "session-1", "appbase"),
            )
            .await
            .expect_err("mismatch");

        assert_eq!(WebFrameworkErrorKind::Forbidden, error.kind);
    }

    #[tokio::test]
    async fn custom_api_key_lookup_service_can_resolve_raw_key() {
        #[derive(Clone)]
        struct StaticApiKeyLookupService;

        #[async_trait]
        impl ApiKeyLookupService for StaticApiKeyLookupService {
            async fn lookup_api_key(
                &self,
                credential: &ApiKeyCredential,
            ) -> Result<ApiKeyPrincipalRecord, WebFrameworkError> {
                assert_eq!("plain-secret-key", credential.raw_value);
                Ok(ApiKeyPrincipalRecord {
                    api_key_id: "custom-key-1".to_owned(),
                    tenant_id: "100001".to_owned(),
                    organization_id: Some("0".to_owned()),
                    user_id: "user-custom".to_owned(),
                    app_id: "appbase".to_owned(),
                    environment: WebEnvironment::Prod,
                    deployment_mode: WebDeploymentMode::Saas,
                    data_scope: vec!["tenant".to_owned()],
                    permission_scope: vec!["custom.read".to_owned()],
                    subject_type: Some("api_key".to_owned()),
                    metadata: BTreeMap::new(),
                })
            }
        }

        let resolver = WebRequestParserResolver::new(
            DefaultAuthTokenParser,
            DefaultAccessTokenParser,
            DefaultApiKeyParser,
            StaticApiKeyLookupService,
        );

        let principal = resolver
            .resolve_api_key("plain-secret-key")
            .await
            .expect("principal");

        assert_eq!("100001", principal.tenant_id());
        assert_eq!(Some("custom-key-1"), principal.api_key_id());
    }

    #[tokio::test]
    async fn default_open_api_resolver_accepts_oauth_bearer_claims() {
        let resolver = DefaultOpenApiWebRequestContextResolver::default();
        let principal = resolver
            .resolve_oauth_bearer(
                "token_id=tok-1;tenant_id=100001;user_id=30;app_id=appbase;environment=prod",
            )
            .await
            .expect("oauth principal");

        assert_eq!("100001", principal.tenant_id());
        assert_eq!("30", principal.user_id());
        assert_eq!(WebSubjectType::Service, principal.subject.subject_type);
    }

    #[tokio::test]
    async fn open_api_flexible_detector_selects_oauth_when_only_bearer_present() {
        use axum::http::{HeaderMap, HeaderValue};

        let detector = DefaultOpenApiCredentialSchemeDetector::default();
        let credentials = WebCallCredentials {
            auth_token: Some("oauth-token".to_owned()),
            access_token: None,
            api_key: None,
            oauth_bearer: Some("oauth-token".to_owned()),
        };
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION_HEADER,
            HeaderValue::from_static("Bearer oauth-token"),
        );
        let scheme = detector
            .detect(&credentials, &headers, Some(RouteAuth::OpenApiFlexible))
            .expect("detect")
            .expect("scheme");
        assert_eq!(OpenApiAuthScheme::OAuthBearer, scheme);
    }
}
