use std::{fs, io, path::Path};
use std::fs::File;
use std::boxed::Box;
use std::error::Error as StdError;
use gltf;
use gltf::buffer::Source;
use gltf::buffer::{View};
use gltf::Accessor;
use gltf::accessor::{DataType, Dimensions};

pub fn load_gltf(dir: &str, file_name: &str) -> Result<gltf::Gltf, Box<StdError>> {

    let path = Path::new(dir).join(file_name);
    let file = fs::File::open(&path).expect("Could not find gltf file");
    let reader = io::BufReader::new(file);
    let gltf = gltf::Gltf::from_reader(reader)?;
    Ok(gltf)
}

fn load_from_source(source: &Source, dir: &str) -> Vec<u8> {

    use std::io::Read;

    let mut v = Vec::new();
    match source {
        Source::Uri(file_name) => {
            let path = Path::new(dir).join(file_name);
            let mut file = File::open(path).expect("Couldn't find file");
            file.read_to_end(&mut v);
        },
        Source::Bin => {
            panic!("loading buffer from embedded binary not implemented");
        },
    };

    v

}

fn extract_vertices_or_normals(accessor: &Accessor, buffers: Vec<Vec<u8>>) -> Vec<glm::Vec4> {

    let mut bd = buffers.clone();
    let buffer = bd.remove(accessor.view().buffer().index());

    match (accessor.data_type(), accessor.dimensions()) {
        (DataType::F32, Dimensions::Vec3) => {
            let offset = accessor.view().offset() + accessor.offset();
            let data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
            println!("buffer len {:?}", data.len());
            let vec = bytes_to_f32(data);
            vec.chunks(3)
                .map(|chunk| glm::vec4(chunk[0], chunk[1], chunk[2], 1.0))
                .collect()
        },
        _ => panic!("could not extract vertices or normals")
    }

}

fn extract_indices(accessor: &Accessor, buffers: Vec<Vec<u8>>) -> Vec<u32> {

    let mut bd = buffers.clone();
    let buffer = bd.remove(accessor.view().buffer().index());

    match (accessor.data_type(), accessor.dimensions()) {
        (DataType::U16, Dimensions::Scalar) => {
            let offset = accessor.view().offset() + accessor.offset();
            let data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
            println!("{:?}", data.len());
            bytes_to_u32(data, 2)
        },
        _ => panic!("could not extract indices")
    }

}
//
//fn extract_data(buffer_data: &Vec<Vec<u8>>, views: &Vec<View>, accessor: &Accessor) -> Vec<u8> {
//
//    let mut bd = buffer_data.clone();
//    let mut buffer = bd.remove(accessor.view().buffer().index());
//
//    match (accessor.data_type(), accessor.dimensions()) {
//        (DataType::U16, Dimensions::Scalar) => {
//            let offset = accessor.view().offset() + accessor.offset();
//            let mut data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
//            println!("{:?}", data.len());
//            //println!("{:?}", data);
//            let bytes = bytes_to_u32(data, 2);
//            println!("{:?}", bytes.len());
//            //println!("{:?}", bytes);
//            u32_to_bytes(bytes)
//        },
//        (DataType::F32, Dimensions::Vec3) => {
//            let offset = accessor.view().offset() + accessor.offset();
//            let mut data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
//            println!("{:?}", data.len());
//            //println!("{:?}", data);
//            let vecf32 = bytes_to_f32(data, 4);
//            let bytes = f32_to_vec4_bytes(vecf32);
//            println!("{:?}", bytes.len());
//            //println!("{:?}", bytes);
//           //bytes
//
//        },
//        _ => {
//            let offset = accessor.view().offset() + accessor.offset();
//            let mut data: Vec<u8> = buffer[offset..(offset + accessor.size() * accessor.count())].to_vec();
//            let clone = data.clone();
//            println!("{:?}", data.len());
//            //println!("{:?}", data);
//            let bytes = bytes_to_f32(clone, 4);
//            println!("{:?}", bytes.len());
//            //println!("{:?}", bytes);
//
//            data
//        },
//    }
//
//
//    buffer
//
//}


fn bytes_to_f32(bytes: Vec<u8>) -> Vec<f32> {
    use byteorder::{LittleEndian, ByteOrder};

    let mut result: Vec<f32> = Vec::new();

    for i in (0..bytes.len()).step_by(4) {
        result.push(LittleEndian::read_f32(&bytes[i..]));
    }

    result

}


fn bytes_to_u32(bytes: Vec<u8>, step: usize) -> Vec<u32> {
    use byteorder::{LittleEndian, ByteOrder};

    let mut result: Vec<u16> = Vec::new();

    for i in (0..bytes.len()).step_by(step) {
        result.push(LittleEndian::read_u16(&bytes[i..]));
    }

    result.into_iter().map(|value|{
        value as u32
    }).collect()


}

fn f32_to_vec4_bytes(vec: Vec<f32>) -> Vec<u8> {

    let temp: Vec<f32> = vec.chunks(3)
        .map(|chunk|{
            glm::vec4(chunk[0], chunk[1], chunk[2], 1.0)
        })
        .map(|vec4|{
            vec4.data.to_vec()
        })
        .flatten()
        .collect();

    f32_to_bytes(temp)

}



fn u32_to_bytes(vec: Vec<u32>) -> Vec<u8> {
    use byteorder::{LittleEndian, ByteOrder};

    let mut result = Vec::new();
    vec.into_iter().for_each(|value|{
        let mut buf: [u8; 4] = [0; 4];
        LittleEndian::write_u32(&mut buf, value);
        result.append(&mut buf.to_vec());
    });
    result
}

fn f32_to_bytes(vec: Vec<f32>) -> Vec<u8> {
    use byteorder::{LittleEndian, ByteOrder};

    let mut result = Vec::new();
    vec.into_iter().for_each(|value|{
        let mut buf: [u8; 4] = [0; 4];
        LittleEndian::write_f32(&mut buf, value);
        result.append(&mut buf.to_vec());
    });
    result
}

pub struct MeshData {
    pub indices: Vec<u32>,
    pub vertices: Vec<glm::Vec4>,
    pub normals: Vec<glm::Vec4>,
}

impl MeshData {

    pub fn no_of_triangles(&self) -> usize {
        self.no_of_indices()/3
    }

    pub fn no_of_indices(&self) -> usize {
        self.indices.len()
    }

    pub fn no_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    pub fn from_gltf(gltf: &gltf::Gltf, dir: &str) -> MeshData {
        let mut sources = Vec::new();

        for b in gltf.buffers() {
            println!("Buffer {:?} info: ", b.name());
            println!("{:?}", b.index());
            println!("{:?}", b.length());
            println!("{:?}", b.source());
            sources.push(b.source());
        }

        //let views: Vec<View> = gltf.views().collect();

        let buffers: Vec<Vec<u8>> = sources
            .iter()
            .map(|source| {
                load_from_source(source, dir)
            }).collect();

        println!("buffer count: {:?}", buffers.len());

//        let mut accessor_data: Vec<(DataType, Vec<u8>)> = gltf.accessors().map(|a| {
//            println!("Accessor {:?} info:", a.index());
//            println!("offset: {:?}", a.offset());
//            println!("count: {:?}", a.count());
//            println!("dimension: {:?}", a.dimensions());
//            println!("size: {:?}", a.size());
//            println!("data type: {:?}", a.data_type());
//            let buf = extract_data(&buffers, &views, &a);
//            (a.data_type(), buf)
//        }).collect();


        let (indices, vertices, normals) = {
            let mut vertices: Vec<glm::Vec4> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();
            let mut normals: Vec<glm::Vec4> = Vec::new();

            for a in gltf.meshes() {
                println!("Mesh name: {:?}", a.name());
                println!("Mesh index: {:?}", a.index());

                for p in a.primitives() {
                    p.attributes().for_each(|(s, a)| {
                        println!("{:?} accessor index: {:?}", s, a.index());
                        println!("accessor offset: {:?}", a.offset());
                        match s {
                            gltf::json::mesh::Semantic::Positions => {
                                vertices = extract_vertices_or_normals(&a, buffers.clone())
                            },
                            gltf::json::mesh::Semantic::Normals => {
                                normals = extract_vertices_or_normals(&a, buffers.clone())
                            },
                            _ => (),
                        }
                    });
                    println!("indices accessor index: {:?}", p.indices().unwrap().index());
                    indices = extract_indices(&p.indices().unwrap(), buffers.clone());
                }
            }

            (indices, vertices, normals)
        };
        println!("indices: {:?}", indices);
        println!("vertices {:?}", vertices);
        println!("normals {:?}", normals);

        MeshData {
            indices,
            vertices,
            normals,
        }
    }
}
