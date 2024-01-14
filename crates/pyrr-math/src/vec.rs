use na::{ComplexField, RealField};
use std::ops::{Add, Mul, Neg, Sub};

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
    fn constructor_vec2f(x: f32, y: f32) -> crate::Vec2f {
        glm::vec2(x, y).into()
    }

    fn static_vec2f_splat(v: f32) -> crate::Vec2f {
        glm::Vec2::repeat(v).into()
    }

    fn method_vec2f_abs(self_: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).abs().into()
    }

    fn method_vec2f_neg(self_: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).neg().into()
    }

    fn method_vec2f_ceil(self_: crate::Vec2f) -> crate::Vec2f {
        glm::ceil(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_floor(self_: crate::Vec2f) -> crate::Vec2f {
        glm::floor(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_trunc(self_: crate::Vec2f) -> crate::Vec2f {
        glm::trunc(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_fract(self_: crate::Vec2f) -> crate::Vec2f {
        glm::fract(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_round(self_: crate::Vec2f) -> crate::Vec2f {
        glm::round(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_signum(self_: crate::Vec2f) -> crate::Vec2f {
        glm::sign(&glm::Vec2::from(self_)).into()
    }

    fn method_vec2f_add(self_: crate::Vec2f, rhs: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).add(&glm::Vec2::from(rhs)).into()
    }

    fn method_vec2f_sub(self_: crate::Vec2f, rhs: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).sub(&glm::Vec2::from(rhs)).into()
    }

    fn method_vec2f_min(self_: crate::Vec2f, rhs: crate::Vec2f) -> crate::Vec2f {
        glm::min2(&glm::Vec2::from(self_), &glm::Vec2::from(rhs)).into()
    }

    fn method_vec2f_max(self_: crate::Vec2f, rhs: crate::Vec2f) -> crate::Vec2f {
        glm::max2(&glm::Vec2::from(self_), &glm::Vec2::from(rhs)).into()
    }

    fn method_vec2f_copysign(self_: crate::Vec2f, sign: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_)
            .zip_map(&glm::Vec2::from(sign), RealField::copysign)
            .into()
    }

    fn method_vec2f_eq(self_: crate::Vec2f, rhs: crate::Vec2f) -> bool {
        glm::Vec2::from(self_).eq(&glm::Vec2::from(rhs))
    }

    fn method_vec2f_ne(self_: crate::Vec2f, rhs: crate::Vec2f) -> bool {
        glm::Vec2::from(self_).ne(&glm::Vec2::from(rhs))
    }

    fn method_vec2f_clamp(
        self_: crate::Vec2f,
        min: crate::Vec2f,
        max: crate::Vec2f,
    ) -> crate::Vec2f {
        glm::clamp_vec(
            &glm::Vec2::from(self_),
            &glm::Vec2::from(min),
            &glm::Vec2::from(max),
        )
        .into()
    }

    fn method_vec2f_powf(self_: crate::Vec2f, pow: f32) -> crate::Vec2f {
        glm::Vec2::from(self_)
            .map(|self_| ComplexField::powf(self_, pow))
            .into()
    }

    fn method_vec2f_powi(self_: crate::Vec2f, pow: i32) -> crate::Vec2f {
        glm::Vec2::from(self_)
            .map(|self_| ComplexField::powi(self_, pow))
            .into()
    }

    fn method_vec2f_recip(self_: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).map(ComplexField::recip).into()
    }

    fn method_vec2f_len(self_: crate::Vec2f) -> f32 {
        glm::Vec2::from(self_).magnitude()
    }

    fn method_vec2f_len_recip(self_: crate::Vec2f) -> f32 {
        ComplexField::recip(Self::method_vec2f_len(self_))
    }

    fn method_vec2f_len_sq(self_: crate::Vec2f) -> f32 {
        glm::Vec2::from(self_).magnitude_squared()
    }

    fn method_vec2f_len_sq_recip(self_: crate::Vec2f) -> f32 {
        ComplexField::recip(Self::method_vec2f_len_sq(self_))
    }

    fn method_vec2f_normalize(self_: crate::Vec2f) -> crate::Vec2f {
        match Self::method_vec2f_normalize_or_wild(self_) {
            it if Self::method_vec2f_is_normalized(it) => it,
            _ => cold_panic!("attempted to normalize vec2f with length zero"),
        }
    }

    fn method_vec2f_normalize_or_zero(self_: crate::Vec2f) -> crate::Vec2f {
        match Self::method_vec2f_normalize_or_wild(self_) {
            it if Self::method_vec2f_is_normalized(it) => it,
            _ => glm::Vec2::zeros().into(),
        }
    }

    fn method_vec2f_normalize_or_wild(self_: crate::Vec2f) -> crate::Vec2f {
        glm::Vec2::from(self_).normalize().into()
    }

    fn method_vec2f_is_normalized(self_: crate::Vec2f) -> bool {
        Self::method_vec2f_is_normalized_enough(self_, X1P_20F)
    }

    fn method_vec2f_is_normalized_enough(self_: crate::Vec2f, epsilon: f32) -> bool {
        glm::is_normalized(&glm::Vec2::from(self_), epsilon)
    }

    fn method_vec2f_project_onto(self_: crate::Vec2f, onto: crate::Vec2f) -> crate::Vec2f {
        let onto = glm::Vec2::from(onto);
        glm::Vec2::from(self_).dot(&onto).mul(onto).into()
    }

    fn method_vec2f_reject_from(self_: crate::Vec2f, onto: crate::Vec2f) -> crate::Vec2f {
        Self::method_vec2f_sub(self_, Self::method_vec2f_project_onto(self_, onto))
    }

    fn method_vec2f_is_nan(self_: crate::Vec2f) -> bool {
        glm::any(&glm::Vec2::from(self_).map(f32::is_nan))
    }

    fn method_vec2f_is_finite(self_: crate::Vec2f) -> bool {
        glm::all(&glm::Vec2::from(self_).map(|self_| ComplexField::is_finite(&self_)))
    }
}
