use crate::core::model::Job;
use crate::core::model::LlmAnalysis;
use crate::core::model::MatchingJobTitles;
use crate::domain::llm::AnalysisResult;
use crate::domain::llm::LLMProvider;
use crate::error::ScannerError::*;
use crate::error::ScannerResult;
use async_trait::async_trait;
use openrouter_rs::{
    OpenRouterClient,
    api::chat::{ChatCompletionRequest, Message},
    types::{Choice, ResponseFormat, Role},
};

pub struct OpenRouterProvider {
    pub openrouter_client: OpenRouterClient,
    pub model_name: String,
}

impl OpenRouterProvider {
    pub fn new(api_key: String, m_name: String) -> ScannerResult<Self> {
        match OpenRouterClient::builder().api_key(api_key).build() {
            Ok(c) => Ok(Self {
                openrouter_client: c,
                model_name: m_name,
            }),
            Err(err) => {
                let err_msg = format!("Cannot build openrouter client: {}", err.to_string());
                Err(LlmError(err_msg))
            }
        }
    }
}

#[async_trait]
impl LLMProvider for OpenRouterProvider {
    async fn filter_titles(
        &self,
        job_list: Vec<Job>,
        profile_summary: &str,
    ) -> ScannerResult<Vec<Job>> {
        let prompt = format!(
            "You are an expert career coach. Based on this professional summary: '{}', review the following list of job titles.
            Return ONLY a JSON array of the titles that are a strong potential match.
            Do not include any other text or explanation. Job Titles: {:?}",
            profile_summary, job_list
        );

        let format = ResponseFormat::json_schema(
            "character_info",
            true,
            serde_json::json!({
              "type": "object",
              "properties": {
                "matching_jobs": {
                  "type": "array",
                  "properties": {
                      "id": {
                        "type": "string",
                        "description": "job id",
                      },
                      "title": {
                        "type": "string",
                        "description": "job title",
                      },
                      "location": {
                        "type": "string",
                        "description": "job location",
                      },
                      "description": {
                        "type": "string",
                        "description": "job description (optional)",
                      },
                      "external_path": {
                        "type": "string",
                        "description": "job externalPath",
                      },
                      "company_name": {
                        "type": "string",
                        "description": "job company name",
                      },
                  },
                }
              },
            }),
        );
        let chat_request = ChatCompletionRequest::builder()
            .model(self.model_name.clone())
            .messages(vec![Message::new(Role::User, &prompt)])
            .response_format(format)
            .build()?;
        let chat_response = self
            .openrouter_client
            .send_chat_completion(&chat_request)
            .await?;
        let choice = chat_response.choices.first().unwrap();
        if let Choice::NonStreaming(non_streaming_choice) = choice {
            if let Some(content) = non_streaming_choice.message.content.clone() {
                let response: MatchingJobTitles =
                    serde_json::from_str(&content).map_err(|e| LlmError(e.to_string()))?;
                return Ok(response.matching_jobs);
            } else {
                return Err(LlmError("something went wrong".to_string()));
            }
        }
        Err(LlmError("something went wrong".to_string()))
    }

    async fn analyze_match(
        &self,
        resume_text: &str,
        job_description: &str,
    ) -> ScannerResult<AnalysisResult> {
        let prompt = format!(
                    "You are an AI hiring assistant. Analyze the following resume and job description.
                    Provide a JSON response with three keys: 'is_match' (boolean),
                    'score' (a number from 0 to 100 representing the quality of the match),
                    and 'reasoning' (a brief, one-paragraph explanation for your decision). Resume: '{}' Job Description: '{}'",
                    resume_text, job_description
                );
        let format = ResponseFormat::json_schema(
            "character_info",
            true,
            serde_json::json!({
              "type": "object",
              "properties": {
                "LlmAnalysis": {
                  "type": "object",
                  "properties": {
                      "is_match": {
                        "type": "bool",
                        "description": "Check if the given job description is suitable match for the profile",
                      },
                      "score": {
                        "type": "number",
                        "description": "score between 0 to 100 on how good is the match",
                      },
                      "reasoning": {
                        "type": "string",
                        "description": "reasoning behind the match",
                      }
                  },
                }
              },
            }),
        );
        let chat_request = ChatCompletionRequest::builder()
            .model(self.model_name.clone())
            .messages(vec![Message::new(Role::User, &prompt)])
            .response_format(format)
            .build()?;
        let chat_response = self
            .openrouter_client
            .send_chat_completion(&chat_request)
            .await?;
        let choice = chat_response.choices.first().unwrap();
        if let Choice::NonStreaming(non_streaming_choice) = choice {
            if let Some(content) = non_streaming_choice.message.content.clone() {
                let response: LlmAnalysis =
                    serde_json::from_str(&content).map_err(|e| LlmError(e.to_string()))?;
                return Ok(AnalysisResult {
                    is_match: response.is_match,
                    score: response.score,
                    reasoning: response.reasoning,
                });
            } else {
                return Err(LlmError("something went wrong".to_string()));
            }
        }
        Err(LlmError("something went wrong".to_string()))
    }
}
