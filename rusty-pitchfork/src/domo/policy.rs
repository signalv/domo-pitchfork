use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Policy {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub policy_type: String,
    #[serde(rename = "users")]
    pub user_ids: Vec<i32>,
    #[serde(rename = "groups")]
    pub group_ids: Vec<i32>,
    pub filters: Vec<Filter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Filter {
    pub column: String,
    pub not: bool,
    pub operator: String,
    pub values: Vec<String>,
}
