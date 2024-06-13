#[derive(Clone)]
pub struct TextFileDescription {
    pub name: String,
    pub extensions: Vec<String>,
    pub comments: Vec<String>,
    pub multiline_comment_start: Vec<String>,
    pub multiline_comment_end: Vec<String>,
}

pub struct AnalyzerConfiguration {
    pub file_types: Vec<TextFileDescription>,
}

impl Default for AnalyzerConfiguration {
    fn default() -> Self {
        Self {
            file_types: vec![TextFileDescription {
                name: "Rust".to_string(),
                extensions: vec!["rs".to_string()],
                comments: vec!["//".to_string()],
                multiline_comment_start: vec!["/*".to_string()],
                multiline_comment_end: vec!["*/".to_string()],
            }],
        }
    }
}

impl AnalyzerConfiguration {
    pub fn load_from_file() -> Self {
        Self { file_types: vec![] }
    }
}