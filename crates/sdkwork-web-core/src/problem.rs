use crate::error::{WebFrameworkError, WebFrameworkErrorKind};
use crate::trace::resolve_problem_trace_id;
use axum::http::{header, HeaderName, HeaderValue};
use axum::response::{IntoResponse, Response};
use serde_json::json;

fn problem_type_uri(kind: &WebFrameworkErrorKind) -> &'static str {
    match kind {
        WebFrameworkErrorKind::MissingCredentials => {
            "https://sdkwork.dev/problems/missing-credentials"
        }
        WebFrameworkErrorKind::InvalidCredentials => {
            "https://sdkwork.dev/problems/invalid-credentials"
        }
        WebFrameworkErrorKind::Forbidden => "https://sdkwork.dev/problems/forbidden",
        WebFrameworkErrorKind::BadRequest => "https://sdkwork.dev/problems/bad-request",
        WebFrameworkErrorKind::Conflict => "https://sdkwork.dev/problems/conflict",
        WebFrameworkErrorKind::PayloadTooLarge => "https://sdkwork.dev/problems/payload-too-large",
        WebFrameworkErrorKind::RateLimitExceeded => {
            "https://sdkwork.dev/problems/rate-limit-exceeded"
        }
        WebFrameworkErrorKind::DependencyUnavailable => {
            "https://sdkwork.dev/problems/dependency-unavailable"
        }
        WebFrameworkErrorKind::RequestTimeout => "https://sdkwork.dev/problems/request-timeout",
        WebFrameworkErrorKind::MethodNotAllowed => {
            "https://sdkwork.dev/problems/method-not-allowed"
        }
        WebFrameworkErrorKind::NotFound => "https://sdkwork.dev/problems/not-found",
        WebFrameworkErrorKind::NotImplemented => "https://sdkwork.dev/problems/not-implemented",
        WebFrameworkErrorKind::InternalServerError => {
            "https://sdkwork.dev/problems/internal-server-error"
        }
        WebFrameworkErrorKind::ContextNotInjected => {
            "https://sdkwork.dev/problems/context-not-injected"
        }
        WebFrameworkErrorKind::WebSocketRejected => {
            "https://sdkwork.dev/problems/websocket-rejected"
        }
    }
}

/// Correlation fields for RFC 7807 Problem+JSON (`OBSERVABILITY_SPEC` §1).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ProblemCorrelation<'a> {
    pub request_id: Option<&'a str>,
    pub trace_id: Option<&'a str>,
}

impl<'a> ProblemCorrelation<'a> {
    pub fn new(request_id: Option<&'a str>, trace_id: Option<&'a str>) -> Self {
        Self {
            request_id,
            trace_id,
        }
    }

    pub fn resolved_trace_id(&self) -> Option<String> {
        if let Some(trace_id) = self.trace_id.filter(|value| !value.is_empty()) {
            return Some(trace_id.to_owned());
        }
        self.request_id
            .map(|request_id| resolve_problem_trace_id(request_id, None))
    }
}

impl<'a> From<Option<&'a str>> for ProblemCorrelation<'a> {
    fn from(trace_id: Option<&'a str>) -> Self {
        Self {
            request_id: None,
            trace_id,
        }
    }
}

/// Client-safe Problem `detail` — internal failures must not leak implementation details.
pub fn client_safe_problem_detail(error: &WebFrameworkError) -> &str {
    match error.kind {
        WebFrameworkErrorKind::InternalServerError => "An internal error occurred",
        WebFrameworkErrorKind::DependencyUnavailable => {
            "A required dependency is temporarily unavailable"
        }
        _ => &error.message,
    }
}

/// Build RFC 9457 Problem+json with required numeric `code` and `traceId`.
pub fn problem_response(
    error: &WebFrameworkError,
    correlation: ProblemCorrelation<'_>,
) -> Response {
    let status = error.status();
    let trace_id = correlation
        .resolved_trace_id()
        .unwrap_or_else(|| "unknown".to_owned());
    let payload = json!({
        "type": problem_type_uri(&error.kind),
        "title": status.canonical_reason().unwrap_or("Request context error"),
        "status": status.as_u16(),
        "code": error.result_code(),
        "traceId": trace_id,
        "detail": client_safe_problem_detail(error),
    });
    let mut response = (status, axum::Json(payload)).into_response();
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/problem+json"),
    );
    if let Ok(value) = HeaderValue::from_str(&trace_id) {
        response.headers_mut().insert(
            HeaderName::from_static(crate::constants::SDKWORK_TRACE_ID_HEADER_LOWER),
            value,
        );
    }
    if let Some(retry_after) = error.retry_after_seconds {
        if let Ok(value) = HeaderValue::from_str(&retry_after.to_string()) {
            response
                .headers_mut()
                .insert(HeaderName::from_static("retry-after"), value);
        }
    }
    response
}

/// Redact numeric/uuid-like path segments for logging (observability spec §10).
pub fn redact_path_template(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if segment.is_empty() {
                return String::new();
            }
            if segment.chars().all(|ch| ch.is_ascii_digit())
                || segment.len() >= 32 && segment.chars().all(|c| c.is_ascii_hexdigit() || c == '-')
            {
                "{id}".to_owned()
            } else {
                segment.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trace::trace_id_from_traceparent;

    #[test]
    fn redacts_numeric_path_segments() {
        assert_eq!(
            "/app/v3/api/users/{id}/orders/{id}",
            redact_path_template("/app/v3/api/users/42/orders/99")
        );
    }

    #[test]
    fn problem_response_sanitizes_internal_errors() {
        let error = WebFrameworkError::internal_server_error("sqlx connection reset by peer");
        let response = problem_response(&error, None.into());
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let bytes = rt
            .block_on(async { axum::body::to_bytes(response.into_body(), usize::MAX).await })
            .expect("body");
        let payload: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
        assert_eq!(
            "An internal error occurred",
            payload["detail"].as_str().unwrap()
        );
        assert!(!payload["detail"].as_str().unwrap().contains("sqlx"));
    }

    #[test]
    fn problem_response_sets_problem_json_content_type() {
        let error = WebFrameworkError::missing_credentials("test");
        let response = problem_response(&error, Some("req-trace-1").into());
        assert_eq!(
            "application/problem+json",
            response
                .headers()
                .get(header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or_default()
        );
    }

    #[test]
    fn problem_response_includes_trace_id() {
        let traceparent = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
        let error = WebFrameworkError::forbidden("denied");
        let response = problem_response(
            &error,
            ProblemCorrelation::new(Some("req-trace"), trace_id_from_traceparent(traceparent)),
        );
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("runtime");
        let bytes = rt
            .block_on(async { axum::body::to_bytes(response.into_body(), usize::MAX).await })
            .expect("body");
        let payload: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
        assert!(payload.get("requestId").is_none());
        assert_eq!(40301, payload["code"].as_i64().unwrap());
        assert_eq!(
            "4bf92f3577b34da6a3ce929d0e0e4736",
            payload["traceId"].as_str().unwrap()
        );
    }

    #[test]
    fn problem_type_uri_is_stable() {
        let error = WebFrameworkError::conflict("dup");
        let response = problem_response(&error, None.into());
        assert!(response.headers().get(header::CONTENT_TYPE).is_some());
    }
}
