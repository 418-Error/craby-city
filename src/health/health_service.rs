use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct HealthService {
    pool: Arc<PgPool>,
}

impl HealthService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn check(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1").execute(self.pool.as_ref()).await?;

        Ok(())
    }
}
