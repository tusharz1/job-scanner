use crate::core::model::Company;
use crate::domain::{job_source::JobSource, llm::LLMProvider, storage::Storage};
use crate::error::ScannerResult;
use std::sync::Arc;

pub async fn process_company<J, L, S>(
    company: &Company,
    resume_text: &str,
    job_source: Arc<J>,
    llm: Arc<L>,
    storage: Arc<S>,
) -> ScannerResult<()>
where
    J: JobSource + Send + Sync,
    L: LLMProvider + Send + Sync,
    S: Storage + Send + Sync,
{
    let jobs = job_source.get_job_list(company).await?;
    let mut filtered_jobs = Vec::new();

    for job in jobs {
        if storage.is_seen(&job.id).await? {
            continue;
        }
        filtered_jobs.push(job);
    }

    let matching_job = llm.filter_titles(filtered_jobs, resume_text).await?;

    for job in matching_job {
        let description = job_source.get_job_details(&job, &company.api_url).await?;
        let analysis = llm.analyze_match(resume_text, &description).await?;
        if analysis.is_match {
            println!(
                "✅ MATCH FOUND: {} at {} (Score: {})",
                job.title, company.name, analysis.score
            );
            println!("Reasoning: {}", analysis.reasoning);
            println!("Apply here: {}{}", company.api_url, job.external_path);
            println!("---");

            storage
                .save_job(&job, analysis.score, analysis.reasoning)
                .await?;
        }

        storage.mark_seen(&job.id).await?;
    }

    Ok(())
}

pub fn hi_processor() {
    println!("Hi from processor");
}
