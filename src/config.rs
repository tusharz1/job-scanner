use crate::{core::model::Company, error::ScannerResult};
use serde::{Deserialize, Serialize};
use std::fs::{self, read_to_string};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub resume_path: String,
    pub llm: LLMChoice,
    pub companies: Vec<Company>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LLMChoice {
    pub model_name: String,
}

#[allow(dead_code)]
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            resume_path: "".to_string(),
            llm: LLMChoice {
                model_name: "mistral:latest".to_string(),
            },
            companies: vec![],
        }
    }
}

impl AppConfig {
    pub fn new() -> ScannerResult<Self> {
        let file_path = Path::new("data/companies.json");
        match fs::exists(file_path) {
            Ok(_) => {
                let content = read_to_string(file_path)?;
                let app_config: AppConfig = serde_json::from_str(&content)?;
                Ok(app_config)
            }
            Err(err) => panic!(
                "Please run the app from which `data/companies.json` can be read {}",
                err
            ),
        }
    }
}
