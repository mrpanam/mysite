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

    pub fn formatted_created_at(&self) -> String {
        // Format as a more readable date for UI
        self.created_at.format("%Y-%m-%d %H:%M").to_string()
    }
}
