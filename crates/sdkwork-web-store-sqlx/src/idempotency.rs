use crate::purge::{sqlx_error, ThrottledPurge};
use crate::{now_epoch_secs, ttl_epoch_secs};
use async_trait::async_trait;
use sdkwork_web_core::{
    IdempotencyBeginOutcome, IdempotencyResponseRecord, IdempotencyStore, WebFrameworkError,
};
use sqlx::SqlitePool;
use std::time::Duration;

pub struct SqlxIdempotencyStore {
    pool: SqlitePool,
    purge: ThrottledPurge,
}

impl SqlxIdempotencyStore {
    pub fn new(pool: SqlitePool) -> Self {
        let purge = ThrottledPurge::idempotency(pool.clone());
        Self { pool, purge }
    }
}

#[async_trait]
impl IdempotencyStore for SqlxIdempotencyStore {
    async fn begin(
        &self,
        key: &str,
        fingerprint: &str,
        ttl: Duration,
    ) -> Result<IdempotencyBeginOutcome, WebFrameworkError> {
        self.purge.maybe_run().await?;
        let row = sqlx::query_as::<_, IdempotencyRow>(
            "SELECT fingerprint, response_status, response_body, content_type \
             FROM web_idempotency_record WHERE idempotency_key = ?",
        )
        .bind(key)
        .fetch_optional(&self.pool)
        .await
        .map_err(sqlx_error)?;

        if let Some(row) = row {
            if row.fingerprint != fingerprint {
                return Err(WebFrameworkError::conflict(
                    "idempotency key was already used with a different request fingerprint",
                ));
            }
            if let Some(status_code) = row.response_status {
                return Ok(IdempotencyBeginOutcome::Replay(IdempotencyResponseRecord {
                    status_code: status_code as u16,
                    body: row.response_body.unwrap_or_default(),
                    content_type: row.content_type,
                }));
            }
            return Err(WebFrameworkError::conflict(
                "idempotency key is already in progress",
            ));
        }

        let now = now_epoch_secs();
        let expires_at = ttl_epoch_secs(ttl);
        let inserted = sqlx::query(
            "INSERT OR IGNORE INTO web_idempotency_record \
             (idempotency_key, fingerprint, response_status, response_body, content_type, created_at, expires_at) \
             VALUES (?, ?, NULL, NULL, NULL, ?, ?)",
        )
        .bind(key)
        .bind(fingerprint)
        .bind(now)
        .bind(expires_at)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error)?;

        if inserted.rows_affected() == 0 {
            return self.begin(key, fingerprint, ttl).await;
        }
        Ok(IdempotencyBeginOutcome::Leader)
    }

    async fn complete(
        &self,
        key: &str,
        fingerprint: &str,
        record: IdempotencyResponseRecord,
        ttl: Duration,
    ) -> Result<(), WebFrameworkError> {
        let expires_at = ttl_epoch_secs(ttl);
        let updated = sqlx::query(
            "UPDATE web_idempotency_record \
             SET response_status = ?, response_body = ?, content_type = ?, expires_at = ? \
             WHERE idempotency_key = ? AND fingerprint = ? AND response_status IS NULL",
        )
        .bind(i64::from(record.status_code))
        .bind(record.body)
        .bind(record.content_type)
        .bind(expires_at)
        .bind(key)
        .bind(fingerprint)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error)?;

        if updated.rows_affected() == 0 {
            let row = sqlx::query_as::<_, IdempotencyRow>(
                "SELECT fingerprint, response_status, response_body, content_type \
                 FROM web_idempotency_record WHERE idempotency_key = ?",
            )
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(sqlx_error)?;
            let Some(row) = row else {
                return Err(WebFrameworkError::bad_request(
                    "idempotency key was not reserved",
                ));
            };
            if row.fingerprint != fingerprint {
                return Err(WebFrameworkError::conflict(
                    "idempotency key fingerprint mismatch while completing response",
                ));
            }
        }
        Ok(())
    }

    async fn release(&self, key: &str, fingerprint: &str) -> Result<(), WebFrameworkError> {
        sqlx::query(
            "DELETE FROM web_idempotency_record \
             WHERE idempotency_key = ? AND fingerprint = ? AND response_status IS NULL",
        )
        .bind(key)
        .bind(fingerprint)
        .execute(&self.pool)
        .await
        .map_err(sqlx_error)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct IdempotencyRow {
    fingerprint: String,
    response_status: Option<i64>,
    response_body: Option<Vec<u8>>,
    content_type: Option<String>,
}
