use crate::core::model::{Company, Job};
use crate::error::ScannerResult;
use async_trait::async_trait;

#[async_trait]
pub trait JobSource {
    async fn get_job_list(&self, company: &Company) -> ScannerResult<Vec<Job>>;
    async fn get_job_details(&self, job: &Job, url: &str) -> ScannerResult<String>;
}
