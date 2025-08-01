use serde::{Deserialize, Serialize};

use crate::core::model::Company;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppConfig {
    pub llm: LLMChoice,
    pub companies: Vec<Company>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LLMChoice {
    model_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            llm: LLMChoice {
                model_name: "mistral:latest".to_string(), //TODO: model name not getting picked from here
            },
            companies: vec![
             Company {
                 name: "athena health".to_string(),
                 api_url: "https://athenahealth.wd1.myworkdayjobs.com/wday/cxs/athenahealth/External".to_string(),
             }
            ,Company {
                name: "adobe".to_string(),
                api_url: "https://autodesk.wd1.myworkdayjobs.com/wday/cxs/autodesk/Ext".to_string(),
            }
            ,Company {
                name: "red hat".to_string(),
                api_url: "https://redhat.wd5.myworkdayjobs.com/wday/cxs/redhat/Jobs".to_string(),
            }
            ,Company {
                name: "blackrock".to_string(),
                api_url: "https://blackrock.wd1.myworkdayjobs.com/wday/cxs/blackrock/BlackRock_Professional".to_string(),
            }],
        }
    }
}
