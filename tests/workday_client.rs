use job_scanner::adapters::workday_client::WorkdayJobPosting;

#[test]
fn test_parse_workday_job_postings_successfully() {
    let sample_json = r#"
        {
            "total": 1,
            "jobPostings": [
                {
                    "title": "Senior Rust Developer",
                    "externalPath": "/job/abc-corp/Rust-Developer-123",
                    "locationsText": "Pune, India"
                }
            ]
        }
        "#;
    let result = serde_json::from_str::<WorkdayJobPosting>(sample_json);
    assert!(result.is_ok());
    let workday_posting = result.unwrap();
    assert_eq!(workday_posting.total, 1);
    assert_eq!(
        workday_posting.jobPostings[0].title,
        "Senior Rust Developer"
    );
}
