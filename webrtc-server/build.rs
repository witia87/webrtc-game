extern crate prost_build;

use std::io::Result;
use std::path::{PathBuf};

fn main() -> Result<()> {
    let mut prototypes: Vec<PathBuf> = Vec::new();
    for element in std::path::Path::new("../proto/").read_dir().unwrap() {
        let path = element.unwrap().path();
        if let Some(extension) = path.extension() {
            if extension == "proto" {
                prototypes.push(path);
            }
        }
    }

    prost_build::Config::new().out_dir("src/messages/")
        .compile_protos(
            faxvec.as_slice(),
            &[PathBuf::from("../proto/")])?;
    Ok(())
}
