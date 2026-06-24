use crate::purge::{sqlx_error, ThrottledPurge};
use crate::{now_epoch_secs, ttl_epoch_secs};
use async_trait::async_trait;
use sdkwork_web_core::{RateLimitStore, WebFrameworkError};
use sqlx::SqlitePool;
use std::time::Duration;

pub struct SqlxRateLimitStore {
    pool: SqlitePool,
    purge: ThrottledPurge,
}

impl SqlxRateLimitStore {
    pub fn new(pool: SqlitePool) -> Self {
        let purge = ThrottledPurge::rate_limit(pool.clone());
        Self { pool, purge }
    }
}

#[derive(sqlx::FromRow)]
struct RateLimitRow {
    request_count: i64,
    window_start: i64,
}

#[async_trait]
impl RateLimitStore for SqlxRateLimitStore {
    async fn check_and_record(
        &self,
        key: &str,
        max_requests: u32,
        window: Duration,
    ) -> Result<(), WebFrameworkError> {
        self.purge.maybe_run().await?;
        let window_secs = window.as_secs().max(1) as i64;
        let now = now_epoch_secs();
        let expires_at = ttl_epoch_secs(window);

        let mut tx = self.pool.begin().await.map_err(sqlx_error)?;

        let row = sqlx::query_as::<_, RateLimitRow>(
            "SELECT request_count, window_start FROM web_rate_limit_bucket WHERE bucket_key = ?",
        )
        .bind(key)
        .fetch_optional(&mut *tx)
        .await
        .map_err(sqlx_error)?;

        let (next_count, window_start) = match row {
            None => (1_i64, now),
            Some(row) if now.saturating_sub(row.window_start) >= window_secs => (1, now),
            Some(row) => {
                if row.request_count >= i64::from(max_requests) {
                    tx.rollback().await.map_err(sqlx_error)?;
                    return Err(WebFrameworkError::rate_limit_exceeded(
                        "rate limit exceeded",
                        window_secs as u64,
                    ));
                }
                (row.request_count + 1, row.window_start)
            }
        };

        if next_count > i64::from(max_requests) {
            tx.rollback().await.map_err(sqlx_error)?;
            return Err(WebFrameworkError::rate_limit_exceeded(
                "rate limit exceeded",
                window_secs as u64,
            ));
        }

        sqlx::query(
            "INSERT INTO web_rate_limit_bucket (bucket_key, request_count, window_start, expires_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(bucket_key) DO UPDATE SET
               request_count = excluded.request_count,
               window_start = excluded.window_start,
               expires_at = excluded.expires_at",
        )
        .bind(key)
        .bind(next_count)
        .bind(window_start)
        .bind(expires_at)
        .execute(&mut *tx)
        .await
        .map_err(sqlx_error)?;

        tx.commit().await.map_err(sqlx_error)?;
        Ok(())
    }
}
