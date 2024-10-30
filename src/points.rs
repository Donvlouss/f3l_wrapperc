pub mod imp;

/// Represent a 4d Point.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WPoint4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
/// Represent a 3d Point.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WPoint3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
/// Represent a 2d Point.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct WPoint2 {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(8))]
pub struct WPoint4_8 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(8))]
pub struct WPoint3_8 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(8))]
pub struct WPoint2_8 {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct WPoint4_16 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct WPoint3_16 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[derive(Debug, Clone, Copy)]
#[repr(C, align(16))]
pub struct WPoint2_16 {
    pub x: f32,
    pub y: f32,
}