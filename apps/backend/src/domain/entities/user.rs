use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(name: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            name,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_id(
        id: i32,
        name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Some(id),
            name,
            created_at,
            updated_at,
        }
    }
}
