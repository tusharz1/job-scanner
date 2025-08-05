use job_scanner::adapters::ollama::OllamaProvider;
use job_scanner::adapters::sqlite_storage::SqliteStorage;
use job_scanner::adapters::workday_client::WorkdayClient;
use job_scanner::{config::AppConfig, core::processor, error::ScannerResult};
use std::time::Duration;
use tracing::error;

#[tokio::main]
async fn main() -> ScannerResult<()> {
    tracing_subscriber::fmt::init();

    let settings = AppConfig::new()?;
    let bytes = std::fs::read(settings.resume_path).unwrap();
    let resume_content: String = pdf_extract::extract_text_from_mem(&bytes).unwrap();

    let workday_client = std::sync::Arc::new(WorkdayClient::new());
    let ollama_provider = std::sync::Arc::new(OllamaProvider::new(settings.llm.model_name));
    let storage = std::sync::Arc::new(SqliteStorage::new("job_scanner.db")?);

    loop {
        for company in &settings.companies {
            if let Err(e) = processor::process_company(
                &company,
                &resume_content,
                workday_client.clone(),
                ollama_provider.clone(),
                storage.clone(),
            )
            .await
            {
                error!("Error processing company {}: {}", company.name, e);
            }
        }
        println!("Finished set");
        tokio::time::sleep(Duration::from_secs(3600)).await; // Run every hour
    }
}
