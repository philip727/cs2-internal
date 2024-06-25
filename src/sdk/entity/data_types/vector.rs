#[derive(Debug)]
#[repr(C)]
pub struct Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug)]
#[repr(align(16), C)]
pub struct VectorAligned3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
