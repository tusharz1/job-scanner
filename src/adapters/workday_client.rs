use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::error::{ScannerError, ScannerResult};
use crate::{
    core::model::{Company, Job},
    domain::job_source::JobSource,
};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WorkdayJobInfo {
    pub title: String,
    pub externalPath: String,
    pub locationsText: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct WorkdayJobPosting {
    pub total: i32,
    pub jobPostings: Vec<WorkdayJobInfo>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct JobPostingInfo {
    pub title: String,
    pub jobDescription: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct JobPostingDetails {
    pub jobPostingInfo: JobPostingInfo,
}

pub struct WorkdayClient {
    client: Client,
    limit: u16,
    offset: u16,
}

impl WorkdayClient {
    pub fn new() -> Self {
        WorkdayClient {
            client: Client::new(),
            limit: 20,
            offset: 0,
        }
    }
    pub fn next_offset(&mut self) -> &Self {
        self.offset += self.limit;
        self
    }

    pub fn change_limit(&mut self, new_limit: u16) -> &Self {
        self.limit = new_limit;
        self
    }
}

#[async_trait]
impl JobSource for WorkdayClient {
    async fn get_job_list(&self, company: &Company) -> ScannerResult<Vec<Job>> {
        let url = format!("{}/jobs", company.api_url);
        let body = serde_json::json!({
            "appliedFacets": {"locationCountry" : ["c4f78be1a8f14da0ab49ce1162348a5e"]}, // filter for India location. TODO: Make these JobSource arguments
            "limit": self.limit,
            "offset": self.offset,
            "searchText": ""
        });
        let resp = self.client.post(&url).json(&body).send().await?;
        match resp.json::<WorkdayJobPosting>().await {
            Ok(posts) => {
                let jobs = posts
                    .jobPostings
                    .iter()
                    .map(|j_info| Job {
                        id: j_info
                            .externalPath
                            .split('/')
                            .last()
                            .unwrap_or("")
                            .to_string(),
                        title: j_info.title.clone(),
                        location: j_info.locationsText.clone(),
                        description: None,
                        company_name: company.name.clone(),
                        external_path: j_info.externalPath.clone(),
                    })
                    .collect();
                Ok(jobs)
            }
            Err(err) => Err(ScannerError::ApiError(err)),
        }
    }

    async fn get_job_details(&self, job: &Job, base_url: &str) -> ScannerResult<String> {
        let url = format!("{}{}", base_url, job.external_path);
        let resp = self.client.get(&url).send().await?;
        let text = resp.text().await?;
        let job_des = serde_json::from_str::<JobPostingDetails>(&text)?;
        Ok(job_des.jobPostingInfo.jobDescription)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_job_posting_details() {
        let json_struct = r#"
            {
                "jobPostingInfo": {
                    "id": "b87a7c2ed4c410010b13642c10b40000",
                    "title": "Associate - Software Engineering",
                    "jobDescription": "Some job description",
                    "location": "B3G - Skyline Belgrade, Kneza Milosa 88, Belgrade",
                    "postedOn": "Posted Yesterday"
               }
          }
            "#;
        let result = serde_json::from_str::<JobPostingDetails>(&json_struct);
        assert!(result.is_ok());
        let job_posting = result.unwrap();
        assert_eq!(
            job_posting.jobPostingInfo.jobDescription,
            "Some job description"
        );
    }
}
