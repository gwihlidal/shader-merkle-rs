use parser::MatchResult;
use std::path::Path;
use utilities;

pub fn path_resolve(
	references: &Vec<MatchResult>,
	root_dir: &Path,
	file_dir: &Path,
) -> Vec<MatchResult> {
	let mut result = references.clone();
	for reference in &mut result {
		let parent_path = if reference.relative_path {
			file_dir
		} else {
			root_dir
		};

		let full_path = parent_path.join(&reference.include_path);
		match full_path.canonicalize() {
			Ok(resolved) => {
				reference.include_path = utilities::string_from_path(&resolved).unwrap();
			}
			Err(err) => {
				println!("Error resolving path! {:?} - {:?}", full_path, err);
			}
		}
	}

	result
}

pub fn path_dedup(references: &Vec<MatchResult>) -> Vec<MatchResult> {
	// Assume all paths have been expanded to their absolute form
	let mut result = references.clone();
	result.sort_by(|a, b| a.include_path.cmp(&b.include_path));
	result.dedup_by(|a, b| a.include_path.eq(&b.include_path));
	result
}

pub fn range_sort(references: &Vec<MatchResult>) -> Vec<MatchResult> {
	let mut result = references.clone();
	result.sort_by(|a, b| a.range.start.cmp(&b.range.start));
	result
}

pub fn strip_base(root_dir: &Path, references: &Vec<MatchResult>) -> Vec<MatchResult> {
	let prefix = root_dir.canonicalize().unwrap();
	let mut result = references.clone();
	for reference in &mut result {
		let include_path = reference.include_path.to_owned();
		let include_path = Path::new(&include_path).strip_prefix(&prefix).unwrap();
		reference.include_path = utilities::string_from_path(&include_path).unwrap();
	}
	result
}