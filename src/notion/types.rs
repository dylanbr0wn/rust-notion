use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Page {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cursor {
    pub stack: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockFormat {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_aspect_ratio: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_full_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_page_width: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_preserve_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_width: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_ratio: Option<f32>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PageBody {
    pub limit: u16,
    pub chunkNumber: u16,
    pub verticalColumns: bool,
    pub pageId: String,
    pub cursor: Cursor,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Title {
    Reg(String),
    Anchor(Vec<Vec<String>>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockValueProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Vec<Vec<serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Vec<Vec<String>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Vec<Vec<String>>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BlockValue {
    pub alive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<String>>,
    pub created_by_id: String,
    pub created_by_table: String,
    pub created_time: u64,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<BlockFormat>,
    pub last_edited_by_id: String,
    pub last_edited_by_table: String,
    pub last_edited_time: u64,
    pub parent_id: String,
    pub parent_table: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<BlockValueProperties>,
    pub space_id: String,

    #[serde(rename = "type")]
    pub kind: String,
    pub version: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    pub role: String,
    pub value: BlockValue,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PageRecordMap {
    pub block: indexmap::IndexMap<String, Block>,
}
#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PageResponse {
    pub cursor: Cursor,
    pub recordMap: PageRecordMap,
}
