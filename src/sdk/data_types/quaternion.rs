#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Debug, Clone, Copy)]
#[repr(align(16))]
pub struct QuaternionAligned {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
