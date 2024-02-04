use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f32) -> Self {
        Sphere {
            position,
            radius: radius.abs(),
        }
    }
}

pub fn init_spheres() -> Vec<Sphere> {
    vec![
        Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0),
        Sphere::new(Vec3::new(4.0, 0.0, 5.0), 2.0),
    ]
}