use job_scanner::{
    adapters::{
        llm::openrouter::OpenRouterProvider, sqlite_storage::SqliteStorage,
        workday_client::WorkdayClient,
    },
    core::{model::Company, processor::process_company},
};
use std::sync::Arc;

#[tokio::test]
async fn process_test() {
    let athena_company = Company {
        name: "athena health".to_string(),
        url: "https://athenahealth.wd1.myworkdayjobs.com/wday/cxs/athenahealth/External"
            .to_string(),
        locations : None,
        locationCountry : None
    };
    let resume_text = "A software backend engineer with 4 years of experince in Java and React";
    let job_source = std::sync::Arc::new(WorkdayClient::new());
    let api_key = std::env::var("OPENAI_API_KEY").unwrap();
    let op = OpenRouterProvider::new(api_key, "qwen/qwen3-30b-a3b:free".to_string()).unwrap();
    let l = std::sync::Arc::new(op);
    match SqliteStorage::new("job_scanner.db") {
        Ok(sqlite) => {
            let storage = Arc::new(sqlite);
            let res = process_company(
                &athena_company,
                &resume_text,
                job_source.clone(),
                l.clone(),
                storage.clone(),
            )
            .await;
            match res {
                Err(err) => panic!("{:?}", err),
                _ => (),
            }
        }
        Err(_) => panic!("bad"),
    }
}
