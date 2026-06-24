//! Idempotency fingerprinting and replay response helpers (catalog D5–D7).

use crate::error::WebFrameworkError;
use crate::extractors::header_value;
use crate::hashing::hash_key_material;
use axum::body::Body;
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::Response;
use serde::{Deserialize, Serialize};

/// Cached HTTP response for idempotent command replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdempotencyResponseRecord {
    pub status_code: u16,
    pub body: Vec<u8>,
    pub content_type: Option<String>,
}

/// Result of reserving an idempotency key before handler execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdempotencyBeginOutcome {
    /// First request with this key — execute handler and cache the response.
    Leader,
    /// Prior completed request — replay cached response without running the handler.
    Replay(IdempotencyResponseRecord),
}

pub fn content_length_from_headers(headers: &HeaderMap) -> Option<u64> {
    headers
        .get(header::CONTENT_LENGTH)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse().ok())
}

/// Resolve idempotency fingerprint — prefers explicit body hash headers (D6).
pub fn resolve_idempotency_fingerprint(
    method: &str,
    path: &str,
    operation_id: Option<&str>,
    headers: &HeaderMap,
    require_body_hash_when_payload: bool,
) -> Result<String, WebFrameworkError> {
    if let Some(explicit) = header_value(headers, crate::constants::IDEMPOTENCY_FINGERPRINT_HEADER)
        .or_else(|| header_value(headers, crate::constants::CONTENT_SHA256_HEADER))
    {
        return Ok(hash_key_material(&format!(
            "{method}:{path}:body:{explicit}"
        )));
    }

    let content_length = content_length_from_headers(headers);
    let has_chunked_body = headers
        .get(header::TRANSFER_ENCODING)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.eq_ignore_ascii_case("chunked"));
    if require_body_hash_when_payload && (content_length.unwrap_or(0) > 0 || has_chunked_body) {
        return Err(WebFrameworkError::bad_request(
            "requests with a body must include X-Content-SHA256 or X-Idempotency-Fingerprint for idempotent commands",
        ));
    }

    Ok(idempotency_fingerprint(
        method,
        path,
        content_length,
        operation_id,
    ))
}

/// Stable fingerprint for method + path + operation + payload size metadata.
pub fn idempotency_fingerprint(
    method: &str,
    path: &str,
    content_length: Option<u64>,
    operation_id: Option<&str>,
) -> String {
    hash_key_material(&format!(
        "{method}:{path}:op={}:len={}",
        operation_id.unwrap_or(""),
        content_length.unwrap_or(0)
    ))
}

/// Build a replay response from a cached idempotency record.
pub fn idempotency_replay_response(
    record: &IdempotencyResponseRecord,
    request_id: Option<&str>,
) -> Result<Response, WebFrameworkError> {
    let status = StatusCode::from_u16(record.status_code).map_err(|_| {
        WebFrameworkError::dependency_unavailable("cached idempotency response has invalid status")
    })?;
    let mut response = Response::new(Body::from(record.body.clone()));
    *response.status_mut() = status;
    if let Some(content_type) = &record.content_type {
        if let Ok(value) = HeaderValue::from_str(content_type) {
            response.headers_mut().insert(header::CONTENT_TYPE, value);
        }
    }
    if let Some(request_id) = request_id {
        if let Ok(value) = HeaderValue::from_str(request_id) {
            response
                .headers_mut()
                .insert(crate::constants::REQUEST_ID_HEADER, value);
        }
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

    #[test]
    fn fingerprint_changes_with_content_length() {
        let a = idempotency_fingerprint("POST", "/app/v3/api/orders", Some(10), None);
        let b = idempotency_fingerprint("POST", "/app/v3/api/orders", Some(20), None);
        assert_ne!(a, b);
    }

    #[test]
    fn requires_body_hash_when_payload_present() {
        let mut headers = HeaderMap::new();
        headers.insert(header::CONTENT_LENGTH, "128".parse().unwrap());
        let error =
            resolve_idempotency_fingerprint("POST", "/app/v3/api/orders", None, &headers, true)
                .expect_err("missing body hash");
        assert_eq!(crate::error::WebFrameworkErrorKind::BadRequest, error.kind);
    }

    #[test]
    fn explicit_body_hash_is_used() {
        let mut headers = HeaderMap::new();
        headers.insert(
            crate::constants::CONTENT_SHA256_HEADER,
            "abc123".parse().unwrap(),
        );
        let fp =
            resolve_idempotency_fingerprint("POST", "/app/v3/api/orders", None, &headers, true)
                .expect("fingerprint");
        assert!(fp.len() >= 16);
    }

    #[test]
    fn requires_body_hash_for_chunked_transfer_encoding() {
        let mut headers = HeaderMap::new();
        headers.insert(header::TRANSFER_ENCODING, "chunked".parse().unwrap());
        let error =
            resolve_idempotency_fingerprint("POST", "/app/v3/api/orders", None, &headers, true)
                .expect_err("missing body hash");
        assert_eq!(crate::error::WebFrameworkErrorKind::BadRequest, error.kind);
    }
}
