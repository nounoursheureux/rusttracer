use tobj;
use util::*;
use types::*;
use cgmath::vec3;
use cgmath::prelude::*;
use std::path::Path;
use shape::{Triangle, Shape, Vertex};

#[derive(Copy, Clone)]
pub struct Face {
    pub v1_off: usize,
    pub v2_off: usize,
    pub v3_off: usize
}

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>
}

impl Mesh {
    pub fn load(path: &Path) -> Option<Mesh> {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        let obj = tobj::load_obj(path);
        match obj {
            Ok((models, _)) => {
                let mesh = &models[0].mesh;
                for i in 0..(mesh.positions.len() / 3) {
                    let position = Point3f::new(mesh.positions[i*3], mesh.positions[i*3+1], mesh.positions[i*3+2]);
                    let normal = vec3(mesh.normals[i*3], mesh.normals[i*3+1], mesh.normals[i*3+2]).normalize();
                    vertices.push(Vertex { position: position, normal: normal});
                }
                for i in 0..(mesh.indices.len()/3) {
                    faces.push(Face { v1_off: mesh.indices[i*3] as usize, v2_off: mesh.indices[i*3+1] as usize, v3_off: mesh.indices[i*3+2] as usize});
                }
                Some(Mesh { vertices: vertices, faces: faces})
            },
            Err(_) => None
        }
    }

    // pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
    //     let mut int = None;

    //     for f in self.faces.iter() {
    //         let v1 = self.vertices[f.v1_off];
    //         let v2 = self.vertices[f.v2_off];
    //         let v3 = self.vertices[f.v3_off];
    //         let triangle = Triangle { v1: v1.position, v2: v2.position, v3: v3.position};
    //         let normal = (v1.normal + v2.normal + v3.normal) / 3.0;

    //         if let Some(t) = triangle.get_closest_hit(ray) {
    //             match int {
    //                 None => int = Some(Intersection { position: ray.at(t), normal: normal }),
    //                 Some(i) => if (i.position - ray.origin).magnitude() > t { 
    //                     int = Some(Intersection { position: ray.at(t), normal: normal })
    //                 }
    //             }
    //         }
    //     }
        
    //     int
    // }
}