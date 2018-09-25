use normalize_line_endings::normalized;
use std::fs::File;
use std::io;
use std::io::Read;
use std::iter::FromIterator;
use std::path::Path;

/*pub fn read_file_data<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
	let mut file = File::open(path.as_ref())?;
	let mut data = Vec::new();
	if let Ok(meta) = file.metadata() {
		println!("Calling reserve for {}", meta.len());
		data.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
	}
	file.read_exact(&mut data)?;
	Ok(data)
}*/

pub fn read_file_string<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path.as_ref())?;
    let mut text = String::new();
    if let Ok(meta) = file.metadata() {
        text.reserve(meta.len() as usize); // Safe to truncate, since it's only a suggestion
    }
    file.read_to_string(&mut text)?;
    let text = String::from_iter(normalized(text.chars()));
    Ok(text)
}

pub fn string_from_path(path: &Path) -> Option<String> {
    let path_os_str = path.as_os_str();
    if let Some(path_str) = path_os_str.to_str() {
        Some(path_str.to_string())
    } else {
        None
    }
}
