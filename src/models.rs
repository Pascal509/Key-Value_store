use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct KeyValue {
    pub k: String,
    pub v: String,
}