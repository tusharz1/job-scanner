use job_scanner::{core::model::Company, domain::llm::AnalysisResult};

#[test]
// check that ensures analysisResult values are correctlty stored.
fn roundtrip_analysis_result() {
    let analysis_result = AnalysisResult {
        is_match: true,
        reasoning: "test reasoning".to_string(),
        score: 3u8,
    };
    assert_eq!(analysis_result.is_match, true);
    assert_eq!(analysis_result.reasoning, "test reasoning".to_string());
    assert_eq!(analysis_result.score, 3);
}

#[test]
fn roundtrip_company() {
    let company = Company {
        name: "test company".to_string(),
        url: "test api url".to_string(),
    };
    assert_eq!(company.name, "test company".to_string());
    assert_eq!(company.url, "test api url".to_string());
}
