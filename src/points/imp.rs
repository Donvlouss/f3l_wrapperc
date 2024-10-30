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

imp_convert_2!(WPoint2, Vec2);
imp_convert_2!(WPoint2_8, Vec2);
imp_convert_2!(WPoint2_16, Vec2);
imp_convert_3!(WPoint3, Vec3);
imp_convert_3!(WPoint3_8, Vec3);
imp_convert_3!(WPoint3_16, Vec3);
imp_convert_4!(WPoint4, Vec4);
imp_convert_4!(WPoint4_8, Vec4);
imp_convert_4!(WPoint4_16, Vec4);

