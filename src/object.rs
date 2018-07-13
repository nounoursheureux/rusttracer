use mesh::*;
use types::*;
use cgmath::prelude::*;
use cgmath::vec4;
use shape::{Triangle, Shape, Vertex};
use material::Material;

pub struct Object {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub triangles: Vec<Triangle>,
    pub transform: Matrix4f,
    pub material: Material
}

impl Object {
    pub fn new(mesh: &Mesh, transform: Matrix4f, material: Material)  -> Object {
        let inv_trans = transform.invert().unwrap().transpose();
        let new_vertices: Vec<Vertex> = mesh.vertices.iter().map(|vert| {
            Vertex { 
                position: Point3f::from_homogeneous(transform * vert.position.to_homogeneous()), 
                normal: (inv_trans * vert.normal.extend(0.0)).truncate() 
            }
        }).collect();
        let mut triangles = Vec::new();
        for f in mesh.faces.iter() {
            let v1 = new_vertices[f.v1_off];
            let v2 = new_vertices[f.v2_off];
            let v3 = new_vertices[f.v3_off];
            triangles.push(Triangle::new(v1, v2, v3))
        }
        Object { vertices: new_vertices, faces: mesh.faces.clone(), triangles: triangles, transform: transform, material: material }
    }

    pub fn intersects(&self, ray: Ray) -> Option<Intersection> {
        let mut int = None;

        for tri in self.triangles.iter() {
            if let Some(t) = tri.get_closest_hit(ray) {
                match int {
                    None => int = Some(Intersection { position: ray.at(t), normal: tri.normal(ray.at(t)) }),
                    Some(i) => if (i.position - ray.origin).magnitude() > t {
                        int = Some(Intersection { position: ray.at(t), normal: tri.normal(ray.at(t)) })
                    }
                }
            }
        }

        // for f in self.faces.iter() {
        //     let v1 = self.vertices[f.v1_off];
        //     let v2 = self.vertices[f.v2_off];
        //     let v3 = self.vertices[f.v3_off];
        //     let triangle = Triangle { v1: v1.position, v2: v2.position, v3: v3.position};
        //     let normal = (v1.normal + v2.normal + v3.normal) / 3.0;

        //     if let Some(t) = triangle.get_closest_hit(ray) {
        //         match int {
        //             None => int = Some(Intersection { position: ray.at(t), normal: normal }),
        //             Some(i) => if (i.position - ray.origin).magnitude() > t { 
        //                 int = Some(Intersection { position: ray.at(t), normal: normal })
        //             }
        //         }
        //     }
        // }
        
        int
    }
}