use sqlx::MySqlPool;
use crate::models::KeyValue;

pub async fn set_value(pool: &MySqlPool, key: &str, value: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO kv_store (k, v) VALUES (?, ?) ON DUPLICATE KEY UPDATE v=?")
        .bind(key)
        .bind(value)
        .bind(value)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_value(pool: &MySqlPool, key: &str) -> Result<Option<String>, sqlx::Error> {
    let result = sqlx::query_as::<_, KeyValue>("SELECT * FROM kv_store WHERE k = ?")
        .bind(key)
        .fetch_optional(pool)
        .await?;

    Ok(result.map(|entry| entry.v))
}

pub async fn delete_value(pool: &MySqlPool, key: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM kv_store WHERE k = ?")
        .bind(key)
        .execute(pool)
        .await?;
    Ok(())
}