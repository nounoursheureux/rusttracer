use types::*;
use object::Object;
use material::Material;
use cgmath::prelude::*;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Point3f>
}

#[derive(Copy, Clone)]
pub struct SceneIntersection {
    pub object_id: usize,
    pub position: Point3f,
    pub normal: Vec3f,
    pub material: Material
}

impl Scene {
    pub fn new() -> Scene {
        Scene { objects: Vec::new(), lights: Vec::new() }
    }

    pub fn intersects(&self, ray: Ray) -> Option<SceneIntersection> {
        let mut inter_opt = None;

        for (id, obj) in self.objects.iter().enumerate() {
            if let Some(i) = obj.intersects(ray) {
                match inter_opt {
                    None => inter_opt = Some(SceneIntersection { object_id: id, position: i.position, normal: i.normal, material: obj.material }),
                    Some(inter) => if (i.position - ray.origin).magnitude() < (inter.position - ray.origin).magnitude() {
                        inter_opt = Some(SceneIntersection { object_id: id, position: i.position, normal: i.normal, material: obj.material });
                    }
                }
            }
        }

        inter_opt
    }
}