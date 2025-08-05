use crate::core::model::Job;
use crate::error::ScannerResult;
use async_trait::async_trait;

#[async_trait]
pub trait Storage {
    async fn is_seen(&self, job_id: &str) -> ScannerResult<bool>;
    async fn mark_seen(&self, job_id: &str) -> ScannerResult<()>;
    async fn save_job(&self, job: &Job, score: u8, reasoning: String) -> ScannerResult<()>;
}
