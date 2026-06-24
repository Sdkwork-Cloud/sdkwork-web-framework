use crate::health::{ReadinessCheck, ReadinessFuture};
use sqlx::SqlitePool;

/// EP-15: verifies the shared SQLx store is reachable before `/readyz` reports ready.
#[derive(Clone)]
pub struct SqliteReadinessCheck {
    pool: SqlitePool,
}

impl SqliteReadinessCheck {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl ReadinessCheck for SqliteReadinessCheck {
    fn check(&self) -> ReadinessFuture<'_> {
        let pool = self.pool.clone();
        Box::pin(async move {
            sqlx::query("SELECT 1")
                .execute(&pool)
                .await
                .map_err(|error| error.to_string())?;
            Ok(())
        })
    }
}
