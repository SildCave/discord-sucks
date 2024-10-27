use std::fmt::Display;

use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(rename = "c")]
    pub content: String,
    #[serde(rename = "a")]
    pub author_id: i32,
    #[serde(rename = "ca")]
    pub created_at: i64,
    #[serde(rename = "ua")]
    pub updated_at: Option<i64>,
    #[serde(rename = "ch")]
    pub channel_id: i32,
}

impl Message {
    pub fn new(
        id: i64,
        content: String,
        author_id: i32,
        created_at: i64,
        updated_at: Option<i64>,
        channel_id: i32
    ) -> Self {
        Self {
            id,
            content,
            author_id,
            created_at,
            updated_at,
            channel_id,
        }
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_author_id(&self) -> i32 {
        self.author_id
    }

    pub fn get_created_at(&self) -> i64 {
        self.created_at
    }

    pub fn get_channel_id(&self) -> i32 {
        self.channel_id
    }
    
}


impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let created_at = DateTime::from_timestamp(self.created_at, 0).unwrap();
        write!(f, "Message: id: {}\ncontent: {}\nauthor_id: {}\ncreated_at: {}", self.id, self.content, self.author_id, created_at)
    }
}