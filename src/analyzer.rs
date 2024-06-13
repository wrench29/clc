use std::{
    collections::HashMap,
    fmt::Display,
    fs, io,
    path::{Path, PathBuf},
};

use crate::config::{AnalyzerConfiguration, TextFileDescription};

pub struct CodeAnalyzer {
    known_files: HashMap<String, TextFileDescription>,
}

pub struct CodeAnalysisReport {
    rows: HashMap<String, CodeAnalysisReportRow>,
}

struct CodeAnalysisReportRow {
    text_files_extensions: String,
    code_lines_count: u64,
    comment_lines_count: u64,
    empty_lines_count: u64,
    files_count: u64,
}

struct FileAnalysisReport {
    file_type: String,
    code_lines_count: u64,
    comment_lines_count: u64,
    empty_lines_count: u64,
}

impl CodeAnalyzer {
    pub fn new(config: AnalyzerConfiguration) -> Self {
        let mut known_files = HashMap::<String, TextFileDescription>::new();
        for file_description in &config.file_types {
            known_files.insert(file_description.name.to_string(), file_description.clone());
        }
        Self { known_files }
    }
    pub fn analyze_dir(&self, dir_path: &Path) -> io::Result<CodeAnalysisReport> {
        let file_paths = Self::get_files_in_directory(dir_path)?;
        let mut report = CodeAnalysisReport {
            rows: HashMap::new(),
        };
        for file_path in file_paths {
            let file_analysis = self.analyze_file(&file_path)?;
            if let Some(file_analysis) = file_analysis {
                let type_report = self.known_files.get(&file_analysis.file_type).unwrap();
                if report.rows.get(&type_report.name).is_none() {
                    report.rows.insert(
                        type_report.name.to_string(),
                        CodeAnalysisReportRow {
                            code_lines_count: 0,
                            comment_lines_count: 0,
                            empty_lines_count: 0,
                            files_count: 0,
                            text_files_extensions: type_report.extensions.join(", "),
                        },
                    );
                }
                let report_row = report.rows.get_mut(&type_report.name).unwrap();
                report_row.files_count += 1;
                report_row.code_lines_count += file_analysis.code_lines_count;
                report_row.comment_lines_count += file_analysis.comment_lines_count;
                report_row.empty_lines_count += file_analysis.empty_lines_count;
            }
        }
        Ok(report)
    }
    fn analyze_file(&self, file_path: &Path) -> io::Result<Option<FileAnalysisReport>> {
        if let Some(ext) = file_path.extension() {
            let ext = ext.to_str().unwrap();
            let file_type_name = self.find_file_type_by_ext(ext);
            if let Some(file_type_name) = file_type_name {
                let known_file = self.known_files.get(&file_type_name).unwrap();
                let mut file_analysis = FileAnalysisReport {
                    file_type: file_type_name.to_string(),
                    code_lines_count: 0,
                    comment_lines_count: 0,
                    empty_lines_count: 0,
                };
                let file_content = fs::read_to_string(file_path)?;
                let comments = &known_file.comments;
                let multiline_comment_starts = &known_file.multiline_comment_start;
                let multiline_comment_ends = &known_file.multiline_comment_end;
                let mut multiline_comment = false;
                for line in file_content.lines() {
                    if line.trim().len() == 0 {
                        file_analysis.empty_lines_count += 1;
                        continue;
                    }
                    let mut was_comment = false;
                    if !multiline_comment {
                        for comment in multiline_comment_starts {
                            if line.starts_with(comment) {
                                multiline_comment = true;
                            }
                        }
                    }
                    if multiline_comment {
                        for comment in multiline_comment_ends {
                            if line.contains(comment) {
                                multiline_comment = false;
                            }
                        }
                        was_comment = true;
                    }
                    if !multiline_comment {
                        for comment in comments {
                            if line.starts_with(comment) {
                                was_comment = true;
                            }
                        }
                    }

                    if was_comment {
                        file_analysis.comment_lines_count += 1;
                    } else {
                        file_analysis.code_lines_count += 1;
                    }
                }
                return Ok(Some(file_analysis));
            }
        }
        Ok(None)
    }
    fn find_file_type_by_ext(&self, extension: &str) -> Option<String> {
        for known_file in &self.known_files {
            for ext in &known_file.1.extensions {
                if extension == ext {
                    return Some(known_file.0.to_string());
                }
            }
        }
        None
    }
    fn get_files_in_directory(dir_path: &Path) -> io::Result<Vec<PathBuf>> {
        let mut file_paths = Vec::<PathBuf>::new();
        if !dir_path.is_dir() {
            return Ok(file_paths);
        }
        let dir_content = fs::read_dir(dir_path)?;
        for entry in dir_content {
            let entry = entry.expect("directory entry should be accessible");
            let entry_path = entry.path();
            if entry_path.is_dir() {
                let mut files_in_dir = Self::get_files_in_directory(&entry_path)?;
                file_paths.append(&mut files_in_dir);
            } else {
                file_paths.push(entry_path);
            }
        }
        Ok(file_paths)
    }
}

impl Display for CodeAnalysisReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row_key in self.rows.keys() {
            let row = self.rows.get(row_key).unwrap();
            write!(f, "--- {} ({}) ---\n", row_key, row.text_files_extensions)?;
            write!(
                f,
                "Files: {}\nLines:\nCode: {}\t | Empty: {}\t | Comments: {}\n",
                row.files_count,
                row.code_lines_count,
                row.empty_lines_count,
                row.comment_lines_count,
            )?;
        }
        Ok(())
    }
}
