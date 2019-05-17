use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;
use gltf;

fn run(path: &str) -> Result<gltf::Gltf, Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    //println!("{:#?}", asset);
    Ok(gltf)
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let gltf = run(&path).expect("runtime error");

        let mesh: gltf::Mesh = gltf.meshes().next().unwrap();

        println!("{:?}", mesh);

    } else {
        println!("usage: asset-display <FILE>");
    }
}