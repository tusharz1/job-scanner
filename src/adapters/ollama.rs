use crate::core::model::{Job, LlmAnalysis, MatchingJobTitles};
use crate::domain::llm::AnalysisResult;
use crate::domain::llm::LLMProvider;
use crate::error::ScannerError::LlmError;
use crate::error::ScannerResult;
use async_trait::async_trait;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::generation::parameters::{FormatType, JsonStructure};

pub struct OllamaProvider {
    ollama: Ollama,
    model: String,
}

impl OllamaProvider {
    pub fn new(model: String) -> Self {
        Self {
            ollama: Ollama::default(),
            model,
        }
    }
}

#[async_trait]
impl LLMProvider for OllamaProvider {
    async fn filter_titles(
        &self,
        job_list: Vec<Job>,
        profile_summary: &str,
    ) -> ScannerResult<Vec<Job>> {
        let prompt = format!(
            "You are an expert career coach. Based on this professional summary: '{}', review the following list of job titles.
            Return ONLY a JSON array of the titles that are a strong potential match. Do not include any other text or explanation. Job Titles: {:?}",
            profile_summary, job_list
        );

        let format =
            FormatType::StructuredJson(Box::new(JsonStructure::new::<MatchingJobTitles>()));
        let res = self
            .ollama
            .generate(GenerationRequest::new(self.model.clone(), prompt).format(format))
            .await
            .map_err(|e| LlmError(e.to_string()))?;

        let response: MatchingJobTitles =
            serde_json::from_str(&res.response).map_err(|e| LlmError(e.to_string()))?;
        println!("filtering complete");
        Ok(response.matching_jobs)
    }
    async fn analyze_match(
        &self,
        resume_text: &str,
        job_description: &str,
    ) -> ScannerResult<AnalysisResult> {
        let prompt = format!(
                    "You are an AI hiring assistant. Analyze the following resume and job description.
                    Provide a JSON response with three keys: 'is_match' (boolean), 'score' (a number from 0 to 100 representing the quality of the match),
                    and 'reasoning' (a brief, one-paragraph explanation for your decision). Resume: '{}' Job Description: '{}'",
                    resume_text, job_description
                );

        let format = FormatType::StructuredJson(Box::new(JsonStructure::new::<LlmAnalysis>()));
        let res = self
            .ollama
            .generate(GenerationRequest::new(self.model.clone(), prompt).format(format))
            .await
            .map_err(|e| LlmError(e.to_string()))?;

        let response: LlmAnalysis =
            serde_json::from_str(&res.response).map_err(|e| LlmError(e.to_string()))?;
        println!("analyzing compelte");
        Ok(AnalysisResult {
            is_match: response.is_match,
            score: response.score,
            reasoning: response.reasoning,
        })
    }
}
