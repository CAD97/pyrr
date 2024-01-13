use na::{ComplexField, RealField};
use std::ops::{Add, Mul, Neg, Sub};

pub use glm::{vec2 as Vec2f, Vec2 as Vec2f};

pub mod waist {
    pub use crate::ffi::types::Vec2f;
}

impl From<waist::Vec2f> for Vec2f {
    #[inline]
    fn from(v: waist::Vec2f) -> Self {
        Vec2f(v.x, v.y)
    }
}

impl From<Vec2f> for waist::Vec2f {
    #[inline]
    fn from(v: Vec2f) -> Self {
        waist::Vec2f { x: v.x, y: v.y }
    }
}

const X1P_20: f64 = 9.536_743_164_062_5e-7; // 2^-20 ≅ 10^-6
#[allow(clippy::cast_possible_truncation)]
const X1P_20F: f32 = X1P_20 as f32;

#[allow(clippy::missing_assert_message)]
const _: () = {
    assert!(X1P_20 == X1P_20F as f64);
    assert!(X1P_20 == (1.0 / (1 << 20) as f64));
    assert!(X1P_20 <= 1.0e-6);
};

// TODO: template/generate for other vector types/widths, wow
impl crate::ffi::vec2f::Guest for crate::ffi::Exports {
    fn constructor_vec2f(x: f32, y: f32) -> waist::Vec2f {
        Vec2f(x, y).into()
    }

    fn static_vec2f_splat(v: f32) -> waist::Vec2f {
        Vec2f::repeat(v).into()
    }

    fn method_vec2f_abs(self_: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).abs().into()
    }

    fn method_vec2f_neg(self_: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).neg().into()
    }

    fn method_vec2f_ceil(self_: waist::Vec2f) -> waist::Vec2f {
        glm::ceil(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_floor(self_: waist::Vec2f) -> waist::Vec2f {
        glm::floor(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_trunc(self_: waist::Vec2f) -> waist::Vec2f {
        glm::trunc(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_fract(self_: waist::Vec2f) -> waist::Vec2f {
        glm::fract(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_round(self_: waist::Vec2f) -> waist::Vec2f {
        glm::round(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_signum(self_: waist::Vec2f) -> waist::Vec2f {
        glm::sign(&Vec2f::from(self_)).into()
    }

    fn method_vec2f_add(self_: waist::Vec2f, rhs: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).add(&Vec2f::from(rhs)).into()
    }

    fn method_vec2f_sub(self_: waist::Vec2f, rhs: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).sub(&Vec2f::from(rhs)).into()
    }

    fn method_vec2f_min(self_: waist::Vec2f, rhs: waist::Vec2f) -> waist::Vec2f {
        glm::min2(&Vec2f::from(self_), &Vec2f::from(rhs)).into()
    }

    fn method_vec2f_max(self_: waist::Vec2f, rhs: waist::Vec2f) -> waist::Vec2f {
        glm::max2(&Vec2f::from(self_), &Vec2f::from(rhs)).into()
    }

    fn method_vec2f_copysign(self_: waist::Vec2f, sign: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_)
            .zip_map(&Vec2f::from(sign), RealField::copysign)
            .into()
    }

    fn method_vec2f_eq(self_: waist::Vec2f, rhs: waist::Vec2f) -> bool {
        Vec2f::from(self_).eq(&Vec2f::from(rhs))
    }

    fn method_vec2f_ne(self_: waist::Vec2f, rhs: waist::Vec2f) -> bool {
        Vec2f::from(self_).ne(&Vec2f::from(rhs))
    }

    fn method_vec2f_clamp(
        self_: waist::Vec2f,
        min: waist::Vec2f,
        max: waist::Vec2f,
    ) -> waist::Vec2f {
        glm::clamp_vec(&Vec2f::from(self_), &Vec2f::from(min), &Vec2f::from(max)).into()
    }

    fn method_vec2f_powf(self_: waist::Vec2f, pow: f32) -> waist::Vec2f {
        Vec2f::from(self_)
            .map(|self_| ComplexField::powf(self_, pow))
            .into()
    }

    fn method_vec2f_powi(self_: waist::Vec2f, pow: i32) -> waist::Vec2f {
        Vec2f::from(self_)
            .map(|self_| ComplexField::powi(self_, pow))
            .into()
    }

    fn method_vec2f_recip(self_: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).map(ComplexField::recip).into()
    }

    fn method_vec2f_len(self_: waist::Vec2f) -> f32 {
        Vec2f::from(self_).magnitude()
    }

    fn method_vec2f_len_recip(self_: waist::Vec2f) -> f32 {
        ComplexField::recip(Self::method_vec2f_len(self_))
    }

    fn method_vec2f_len_sq(self_: waist::Vec2f) -> f32 {
        Vec2f::from(self_).magnitude_squared()
    }

    fn method_vec2f_len_sq_recip(self_: waist::Vec2f) -> f32 {
        ComplexField::recip(Self::method_vec2f_len_sq(self_))
    }

    fn method_vec2f_normalize(self_: waist::Vec2f) -> waist::Vec2f {
        match Self::method_vec2f_normalize_or_wild(self_) {
            it if Self::method_vec2f_is_normalized(it) => it,
            _ => cold_panic!("attempted to normalize vec2f with length zero"),
        }
    }

    fn method_vec2f_normalize_or_zero(self_: waist::Vec2f) -> waist::Vec2f {
        match Self::method_vec2f_normalize_or_wild(self_) {
            it if Self::method_vec2f_is_normalized(it) => it,
            _ => Vec2f::zeros().into(),
        }
    }

    fn method_vec2f_normalize_or_wild(self_: waist::Vec2f) -> waist::Vec2f {
        Vec2f::from(self_).normalize().into()
    }

    fn method_vec2f_is_normalized(self_: waist::Vec2f) -> bool {
        Self::method_vec2f_is_normalized_enough(self_, X1P_20F)
    }

    fn method_vec2f_is_normalized_enough(self_: waist::Vec2f, epsilon: f32) -> bool {
        glm::is_normalized(&Vec2f::from(self_), epsilon)
    }

    fn method_vec2f_project_onto(self_: waist::Vec2f, onto: waist::Vec2f) -> waist::Vec2f {
        let onto = Vec2f::from(onto);
        Vec2f::from(self_).dot(&onto).mul(onto).into()
    }

    fn method_vec2f_reject_from(self_: waist::Vec2f, onto: waist::Vec2f) -> waist::Vec2f {
        Self::method_vec2f_sub(self_, Self::method_vec2f_project_onto(self_, onto))
    }

    fn method_vec2f_is_nan(self_: waist::Vec2f) -> bool {
        glm::any(&Vec2f::from(self_).map(f32::is_nan))
    }

    fn method_vec2f_is_finite(self_: waist::Vec2f) -> bool {
        glm::all(&Vec2f::from(self_).map(|self_| ComplexField::is_finite(&self_)))
    }
}
