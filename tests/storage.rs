use job_scanner::adapters::sqlite_storage::SqliteStorage;
use job_scanner::core::model::Job;
use job_scanner::domain::storage::Storage;

#[tokio::test]
async fn test_mark_and_check_seen_job() {
    let storage = SqliteStorage::new(":memory:").expect("Failed to create in-memory db");
    let job_id = "test_job_123";
    let is_seen_before = storage.is_seen(job_id).await.unwrap();
    assert!(!is_seen_before, "Job should not be seen initially");
    storage.mark_seen(job_id).await.unwrap();
    let is_seen_after = storage.is_seen(job_id).await.unwrap();
    assert!(is_seen_after, "Job should be seen after marking");
}
#[tokio::test]

async fn test_save_and_retrieve_job() {
    let storage = SqliteStorage::new(":memory:").expect("Failed to create in-memory db");
    let job = Job {
        id: "saved_job_456".to_string(),
        title: "Test Engineer".to_string(),
        company_name: "TestCo".to_string(),
        location: "Remote".to_string(),
        description: None,
        external_path: "/test/path".to_string(),
    };

    storage
        .save_job(&job, 85, "Good fit.".to_string())
        .await
        .unwrap();

    let conn = storage.conn.lock().await;
    let mut stmt = conn
        .prepare("SELECT score, reasoning FROM matched_jobs WHERE id = ?")
        .unwrap();
    let (score, reasoning): (u8, String) = stmt
        .query_row([&job.id], |row| Ok((row.get(0)?, row.get(1)?)))
        .unwrap();

    assert_eq!(score, 85);
    assert_eq!(reasoning, "Good fit.");
}
