use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn load_from_file(path_to_config: &Path) -> io::Result<Self> {
        let config_raw = fs::read_to_string(path_to_config)?;
        let config: Vec<TextFileDescription> =
            serde_yaml::from_str(&config_raw).expect("config format should be valid");
        Ok(Self { file_types: config })
    }
    pub fn find_correct_config_file() -> Option<PathBuf> {
        let config_path = PathBuf::from("formats.yaml");
        if config_path.exists() {
            return Some(config_path);
        }
        let path_with_exec = env::current_exe();
        if let Ok(mut path_with_exec) = path_with_exec {
            path_with_exec.pop();
            let config_path = path_with_exec.join("formats.yaml");
            if config_path.exists() {
                return Some(config_path);
            }
        }
        None
    }
}
