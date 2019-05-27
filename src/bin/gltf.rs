extern crate blackhole;

use blackhole::asset;

use std::{fs, io, path};

use std::fs::File;
use std::boxed::Box;
use std::error::Error as StdError;
use path::Path;
use gltf;
use gltf::buffer::Source;
use gltf::json;
use image::load;

fn run(path: &str) -> Result<gltf::Gltf, Box<StdError>> {
    let file = fs::File::open(&path)?;
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    //println!("{:#?}", asset);
    Ok(gltf)
}

fn load_from_source(source: &Source) -> Vec<u8> {

    use std::io::Read;

    let mut v = Vec::new();
    match source {
        Source::Uri(path) => {
            let mut file = File::open(&path).expect("Couldn't find file");
            file.read_to_end(&mut v);
        },
        Source::Bin => {
            ()
        },
    };

    v

}
//
//fn extract_indices(buffer: vec<u8>, view: gltf::buffer::View, accessor: gltf::accessor::Accessor) -> vec<u32> {
//
//}
fn main() {

    let gltf = run("assets/Box.gltf").expect("runtime error");

    for a in gltf.meshes() {
        println!("Mesh name: {:?}", a.name());
        println!("Mesh index: {:?}", a.index());

        for p in a.primitives() {
            p.attributes().for_each(|(s, a)|{
                println!("accessor semantic: {:?}", s);
                println!("accessor index: {:?}", a.index());
                println!("accessor offset: {:?}", a.offset());
            });
            println!("primitive index: {:?}", p.indices().unwrap().index());
        }

    }

    let mut sources = Vec::new();
    for b in gltf.buffers() {
        println!("Buffer {:?} info: ", b.name());
        println!("{:?}", b.index());
        println!("{:?}", b.length());
        println!("{:?}", b.source());
        sources.push(b.source());

    }

    let buffers: Vec<Vec<u8>> = sources
        .iter()
        .map(|source|{
            load_from_source(&Source::Uri("assets/Box0.bin"))
        }).collect();

    println!("buffer count: {:?}", buffers.len());
    println!("{:?}", buffers);



    for a in gltf.accessors() {
        println!("Accessor {:?} info:", a.index());
        println!("offset: {:?}", a.offset());
        println!("count: {:?}", a.count());
        println!("dimension: {:?}", a.dimensions());
        println!("size: {:?}", a.size());
        println!("size: {:?}", a.data_type());
    }

    let source = "assets/Box.gltf";


//    let (doc, buffers, images) = match gltf::import(source) {
//        Ok(tuple) => tuple,
//        Err(err) => {
//            panic!("glTF import failed: {:?}", err)
////            if let gltf::Error::Io(_) = err {
////                panic!("Hint: Are the .bin file(s) referenced by the .gltf file available?")
////            }
//        },
//    };





}