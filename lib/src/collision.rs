use brdf::Brdf;
use Vec3;

pub struct Collision<'brdf, BrdfType: Brdf + 'static> {
    pub position: Vec3,
    pub normal: Vec3,
    pub brdf: &'brdf BrdfType,
}

impl<'a, BrdfType: Brdf + 'static> Collision<'a, BrdfType> {
    pub fn new(position: Vec3, normal: Vec3, brdf: &'a BrdfType) -> Self {
        Collision {
            position: position,
            normal: normal,
            brdf: brdf
        }
    }
}