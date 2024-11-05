use std::ops::{Index, IndexMut};

use f3l_glam::glam::{Vec2, Vec3, Vec4};

use super::*;

macro_rules! imp_convert_2 {
    ($wp: ty, $v: ty) => {
        impl From<$v> for $wp {
            fn from(p: $v) -> Self {
                Self { x: p.x, y: p.y }
            }
        }
        impl From<$wp> for $v {
            fn from(p: $wp) -> Self {
                Self { x: p.x, y: p.y }
            }
        }
    };
}
macro_rules! imp_convert_3 {
    ($wp: ty, $v: ty) => {
        impl From<$v> for $wp {
            fn from(p: $v) -> Self {
                Self { x: p.x, y: p.y, z: p.z }
            }
        }
        impl From<$wp> for $v {
            fn from(p: $wp) -> Self {
                Self { x: p.x, y: p.y, z: p.z }
            }
        }
    };
}
macro_rules! imp_convert_4 {
    ($wp: ty, $v: ty) => {
        impl From<$v> for $wp {
            fn from(p: $v) -> Self {
                Self { x: p.x, y: p.y, z: p.z, w: p.w }
            }
        }
        impl From<$wp> for $v {
            fn from(p: $wp) -> Self {
                Self::new(p.x, p.y, p.z, p.w)
            }
        }
    };
}
macro_rules! index_2 {
    ($wp: ty) => {
        impl Index<usize> for $wp {
            type Output = f32;
        
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl IndexMut<usize> for $wp {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    _ => panic!("index out of bounds"),
                }
            }
        }
    };
}
macro_rules! index_3 {
    ($wp: ty) => {
        impl Index<usize> for $wp {
            type Output = f32;
        
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl IndexMut<usize> for $wp {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    _ => panic!("index out of bounds"),
                }
            }
        }
    };
}
macro_rules! index_4 {
    ($wp: ty) => {
        impl Index<usize> for $wp {
            type Output = f32;
        
            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    3 => &self.w,
                    _ => panic!("index out of bounds"),
                }
            }
        }
        impl IndexMut<usize> for $wp {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    0 => &mut self.x,
                    1 => &mut self.y,
                    2 => &mut self.z,
                    3 => &mut self.w,
                    _ => panic!("index out of bounds"),
                }
            }
        }
    };
}

imp_convert_2!(WPoint2, Vec2);
imp_convert_2!(WPoint2_8, Vec2);
imp_convert_2!(WPoint2_16, Vec2);
imp_convert_3!(WPoint3, Vec3);
imp_convert_3!(WPoint3_8, Vec3);
imp_convert_3!(WPoint3_16, Vec3);
imp_convert_4!(WPoint4, Vec4);
imp_convert_4!(WPoint4_8, Vec4);
imp_convert_4!(WPoint4_16, Vec4);
index_2!(WPoint2);
index_2!(WPoint2_8);
index_2!(WPoint2_16);
index_3!(WPoint3);
index_3!(WPoint3_8);
index_3!(WPoint3_16);
index_4!(WPoint4);
index_4!(WPoint4_8);
index_4!(WPoint4_16);