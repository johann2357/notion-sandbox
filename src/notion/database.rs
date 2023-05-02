use std::collections::HashMap;

use reqwest;
use serde::{Deserialize, Serialize};

use super::errors::NotionError;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub object: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "type")]
    pub icon_type: String,
    pub emoji: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Parent {
    #[serde(rename = "type")]
    pub parent_type: String,
    pub database_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub object: String,
    pub id: String,
    pub created_time: String,
    pub last_edited_time: String,
    pub created_by: User,
    pub last_edited_by: User,
    pub cover: Option<String>,
    pub icon: Icon,
    pub parent: Parent,
    pub archived: bool,
    pub properties: HashMap<String, serde_json::Value>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub object: String,
    pub results: Vec<Page>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    #[serde(rename = "type")]
    pub response_type: String,
    pub page: HashMap<String, serde_json::Value>,
}

pub async fn find_pages(
    database_id: &String,
    body: serde_json::Value,
    token: &String,
) -> Result<QueryResponse, NotionError> {
    let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Notion-Version", "2022-06-28")
        .header("Authorization", format!("Bearer {}", token))
        .body(body.to_string())
        .send()
        .await
        .map_err(NotionError::Reqwest)?;

    let response_text = response.text().await.map_err(NotionError::Reqwest)?;

    let file: QueryResponse = serde_json::from_str(&response_text).map_err(NotionError::Serde)?;

    return Ok(file);
}
