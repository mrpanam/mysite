use serde::{Deserialize, Serialize};
use surrealdb_types::{Datetime, RecordId, SurrealValue, ToSql};

#[derive(Debug, Clone, Serialize, Deserialize, SurrealValue)]
pub struct Teacher {
    pub id: Option<RecordId>,
    pub name: String,
    pub expertise: String,
    pub created_at: Datetime,
}

impl Teacher {
    pub fn id_key(&self) -> String {
        self.id
            .as_ref()
            .map(|id| id.key.to_sql())
            .unwrap_or_default()
    }
}
