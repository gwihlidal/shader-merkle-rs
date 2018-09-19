extern crate shader_merkle;

use shader_merkle::parser;
use std::path::Path;

fn main() {
    let root_dir = Path::new("./tests/data");
	let shader_file = root_dir.join("CodeGenHLSL/Samples/MiniEngine/ParticleSpawnCS.hlsl");
	let shader_dir = shader_file.parent().unwrap();

    let mut result: Vec<parser::MatchResult> = Vec::new();

    let parser = parser::IncludeParser::new();
    parser.parse_file_recursive(&shader_file, &root_dir, &shader_dir, &mut result);

    /*pub fn parse_file_recursive(
        &self,
        input_file: &Path,
        root_dir: &Path,
        file_dir: &Path,
        result: &mut Vec<MatchResult>,
    ) {*/
    println!("Testing");
}