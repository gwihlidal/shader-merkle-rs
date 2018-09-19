use regex::Regex;
use std::sync::{Arc, RwLock};
use std::path::Path;
use utilities;
use paths::*;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchRange {
    pub start: usize,
	pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchResult {
	pub include_path: String,
    pub range: MatchRange,
	pub relative_path: bool,
}

#[derive(Default, Debug)]
struct IncludeParserState {
    visited: RwLock<Vec<String>>,
}

unsafe impl Send for IncludeParserState {}
unsafe impl Sync for IncludeParserState {}

pub struct IncludeParser {
    state: Arc<IncludeParserState>,
}

impl IncludeParser {
    pub fn new() -> Self {
        IncludeParser {
            state: Arc::new(IncludeParserState::default()),
        }
    }

    pub fn get_matches(&self, text: &str, root_dir: &Path, file_dir: &Path) -> Vec<MatchResult> {
        let raw_matches = self.parse_text(&text);
        let resolved_matches = path_resolve(&raw_matches, &root_dir, &file_dir);
        range_sort(&path_dedup(&resolved_matches))
    }

    pub fn parse_file_recursive(
        &self,
        input_file: &Path,
        root_dir: &Path,
        file_dir: &Path,
        result: &mut Vec<MatchResult>,
    ) {
        let input = utilities::read_file_string(&input_file).unwrap();
        let mut includes = self.get_matches(&input, root_dir, file_dir);

        // Remove includes that were already parsed
        let state = self.state.clone();

        for include in &includes {

            let mut unique = true;
            {
                let mut visited = state.visited.write().unwrap();
                if visited.contains(&include.include_path) {
                    unique = false;
                } else {
                    visited.push(include.include_path.to_owned());
                }
            }

            if unique {
                let mut inner_result = Vec::new();
                let file_dir = Path::new(&include.include_path).parent().unwrap();
                self.parse_file_recursive(&Path::new(&include.include_path), root_dir, file_dir, &mut inner_result);
                result.append(&mut inner_result);
            } else {
                println!("Filtered out cycle: {:?}", include);
            }
        }

        result.append(&mut includes);
    }

    pub fn parse_text(&self, input: &str) -> Vec<MatchResult> {
        //r#"(?m)^*\#include\s+["<]([^">]+)*[">]"#
        //r#"(?m)(^*\#\s*include\s*<([^<>]+)>)|(^\s*\#\s*include\s*"([^"]+)")"#

        lazy_static! {
            static ref ABSOLUTE_PATH_REGEX: Regex = Regex::new(r#"(?m)^*\#\s*include\s*<([^<>]+)>"#)
                .expect("failed to compile absolute include path regex");
        }

        lazy_static! {
            static ref RELATIVE_PATH_REGEX: Regex = Regex::new(r#"(?m)^*\#\s*include\s*"([^"]+)""#)
                .expect("failed to compile relative include path regex");
        }

        let mut results: Vec<MatchResult> = Vec::with_capacity(8);

        // Result will be an iterator over tuples containing the start and end indices for each match in the string
        let absolute_results = ABSOLUTE_PATH_REGEX.find_iter(input);
        for absolute_result in absolute_results {
            let range_start = absolute_result.start();
            let range_end = absolute_result.end();
            let range_text = &input[range_start..range_end];
            let range_caps = ABSOLUTE_PATH_REGEX.captures(range_text).unwrap();
            let include_path = range_caps.get(1).map_or("", |m| m.as_str());
            if include_path.len() > 0 {
                results.push(MatchResult {
                    include_path: include_path.to_owned(),
                    range: MatchRange {
                        start: range_start,
                        end: range_end,
                    },
                    relative_path: false,
                });
            }
        }

        let relative_results = RELATIVE_PATH_REGEX.find_iter(input);
        for relative_result in relative_results {
            let range_start = relative_result.start();
            let range_end = relative_result.end();
            let range_text = &input[range_start..range_end];
            let range_text = range_text.trim().trim_matches('\n');
            let range_caps = RELATIVE_PATH_REGEX.captures(range_text).unwrap();
            let include_path = range_caps.get(1).map_or("", |m| m.as_str());
            if include_path.len() > 0 {
                results.push(MatchResult {
                    include_path: include_path.to_owned(),
                    range: MatchRange {
                        start: range_start,
                        end: range_end,
                    },
                    relative_path: true,
                });
            }
        }

        results
    }
}

