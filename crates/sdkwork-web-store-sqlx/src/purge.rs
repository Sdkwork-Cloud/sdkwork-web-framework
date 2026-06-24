use sqlx::SqlitePool;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use crate::now_epoch_secs;
use sdkwork_web_core::WebFrameworkError;

const DEFAULT_PURGE_INTERVAL_SECS: i64 = 60;

#[derive(Clone)]
pub(crate) struct ThrottledPurge {
    pool: SqlitePool,
    table_sql: Arc<str>,
    last_purge_secs: Arc<AtomicI64>,
    interval_secs: i64,
}

impl ThrottledPurge {
    pub(crate) fn idempotency(pool: SqlitePool) -> Self {
        Self::new(
            pool,
            "DELETE FROM web_idempotency_record WHERE expires_at <= ?",
        )
    }

    pub(crate) fn rate_limit(pool: SqlitePool) -> Self {
        Self::new(
            pool,
            "DELETE FROM web_rate_limit_bucket WHERE expires_at <= ?",
        )
    }

    fn new(pool: SqlitePool, table_sql: &'static str) -> Self {
        Self {
            pool,
            table_sql: Arc::from(table_sql),
            last_purge_secs: Arc::new(AtomicI64::new(0)),
            interval_secs: DEFAULT_PURGE_INTERVAL_SECS,
        }
    }

    pub(crate) async fn maybe_run(&self) -> Result<(), WebFrameworkError> {
        let now = now_epoch_secs();
        let last = self.last_purge_secs.load(Ordering::Relaxed);
        if now.saturating_sub(last) < self.interval_secs {
            return Ok(());
        }
        if self
            .last_purge_secs
            .compare_exchange(last, now, Ordering::SeqCst, Ordering::Relaxed)
            .is_err()
        {
            return Ok(());
        }
        sqlx::query(self.table_sql.as_ref())
            .bind(now)
            .execute(&self.pool)
            .await
            .map_err(sqlx_error)?;
        Ok(())
    }
}

pub(crate) fn sqlx_error(error: sqlx::Error) -> WebFrameworkError {
    WebFrameworkError::dependency_unavailable(format!("sqlx store error: {error}"))
}
