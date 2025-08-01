use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub name: String,
    pub api_url: String,
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct Job {
    pub id: String,
    pub title: String,
    pub location: String,
    pub description: Option<String>,
    pub external_path: String,
    pub company_name: String,
}
