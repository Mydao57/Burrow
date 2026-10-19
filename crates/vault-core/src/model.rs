use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Entry {
    #[zeroize(skip)]
    pub id: Uuid,
    pub title: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    #[serde(with = "time::serde::rfc3339")]
    #[zeroize(skip)]
    pub modified_at: OffsetDateTime,
}

impl Entry {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            username: String::new(),
            password: String::new(),
            url: String::new(),
            notes: String::new(),
            modified_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn touch(&mut self) {
        self.modified_at = OffsetDateTime::now_utc();
    }

    /// Case-insensitive match on title, username and URL (never password or notes).
    pub fn matches(&self, query: &str) -> bool {
        let q = query.trim().to_lowercase();
        q.is_empty()
            || self.title.to_lowercase().contains(&q)
            || self.username.to_lowercase().contains(&q)
            || self.url.to_lowercase().contains(&q)
    }
}

/// The JSON document stored (encrypted) in the vault file.
#[derive(Debug, Default, Serialize, Deserialize)]
pub(crate) struct VaultData {
    pub entries: Vec<Entry>,
}
