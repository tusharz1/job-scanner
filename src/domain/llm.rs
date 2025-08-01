use crate::core::model::Job;
use crate::error::ScannerResult;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct AnalysisResult {
    pub is_match: bool,
    pub reasoning: String,
    pub score: u8,
}

#[async_trait]
pub trait LLMProvider {
    async fn filter_titles(
        &self,
        job_list: Vec<Job>,
        profile_summary: &str,
    ) -> ScannerResult<Vec<Job>>;
    async fn analyze_match(
        &self,
        resume_text: &str,
        job_description: &str,
    ) -> ScannerResult<AnalysisResult>;
}
