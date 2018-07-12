use mesh::*;
use types::*;
use cgmath::prelude::*;
use cgmath::vec4;
use shape::{Triangle, Shape};

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub transform: Matrix4f
}

impl Object {
    pub fn new(mesh: &Mesh, transform: Matrix4f) -> Object {
        let inv_trans = transform.invert().unwrap().transpose();
        let new_vertices: Vec<Vertex> = mesh.vertices.iter().map(|vert| {
            Vertex { 
                position: (transform * vert.position.extend(1.0)).truncate(), 
                normal: (inv_trans * vert.normal.extend(0.0)).truncate() 
            }
        }).collect();
        Object { vertices: new_vertices, faces: mesh.faces.clone(), transform: transform }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
        let mut int = None;

        for f in self.faces.iter() {
            let v1 = self.vertices[f.v1_off];
            let v2 = self.vertices[f.v2_off];
            let v3 = self.vertices[f.v3_off];
            let triangle = Triangle { v1: v1.position, v2: v2.position, v3: v3.position};
            let normal = (v1.normal + v2.normal + v3.normal) / 3.0;

            if let Some(t) = triangle.get_closest_hit(ray) {
                match int {
                    None => int = Some(Intersection { position: ray.at(t), normal: normal }),
                    Some(i) => if (i.position - ray.origin).magnitude() > t { 
                        int = Some(Intersection { position: ray.at(t), normal: normal })
                    }
                }
            }
        }
        
        int
    }
}