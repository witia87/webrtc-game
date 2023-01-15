extern crate prost_build;

use std::io::Result;
use std::path::{PathBuf};

fn main() -> Result<()> {
    build_protobufs("../proto/", "src/messages/")
}

fn build_protobufs(input_directory: &str,
                   output_directory: &str) -> Result<()> {

    let mut prototypes: Vec<PathBuf> = Vec::new();
    for element in std::path::Path::new(input_directory).read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "proto" {
                prototypes.push(path);
            }
        }
    }

    prost_build::Config::new().out_dir(output_directory)
        .compile_protos(
            prototypes.as_slice(),
            &[PathBuf::from(input_directory)])?;
    Ok(())
}
