use crate::core::model::Job;
use crate::domain::storage::Storage;
use crate::error::ScannerResult;
use async_trait::async_trait;
use rusqlite::{Connection, params};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteStorage {
    pub fn new(path: &str) -> ScannerResult<Self> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS seen_jobs (id TEXT PRIMARY KEY, scanned_at DATETIME DEFAULT CURRENT_TIMESTAMP)",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS matched_jobs (id TEXT PRIMARY KEY, title TEXT, company TEXT, location TEXT, score INTEGER, reasoning TEXT
            , created_at DATETIME DEFAULT CURRENT_TIMESTAMP)",
            [],
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn mark_seen(&self, job_id: &str) -> ScannerResult<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR IGNORE INTO seen_jobs (id) VALUES (?)",
            params![job_id],
        )?;
        Ok(())
    }

    async fn is_seen(&self, job_id: &str) -> ScannerResult<bool> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare("SELECT 1 FROM seen_jobs WHERE id = ?")?;
        let exists = stmt.exists([job_id])?;
        Ok(exists)
    }

    async fn save_job(&self, job: &Job, score: u8, reasoning: String) -> ScannerResult<()> {
        let conn = self.conn.lock().await;
        conn.execute(
            "INSERT OR REPLACE INTO matched_jobs (id, title, company, location, score, reasoning) VALUES (?, ?, ?, ?, ?, ?)",
            params![job.id, job.title, job.company_name, job.location, score, reasoning],
        )?;
        Ok(())
    }
}
