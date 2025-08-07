use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Company {
    pub name: String,
    pub url: String,
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

#[derive(JsonSchema, Deserialize, Debug)]
pub struct MatchingJobTitles {
    pub matching_jobs: Vec<Job>,
}

#[derive(JsonSchema, Deserialize, Debug)]
pub struct LlmAnalysis {
    pub is_match: bool,
    pub score: u8,
    pub reasoning: String,
}
