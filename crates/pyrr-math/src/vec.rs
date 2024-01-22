use std::{
    mem::{size_of, transmute_copy, ManuallyDrop},
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

// macro_rules! count_tts {
//     () => { 0 };
//     ($odd:tt $($a:tt $b:tt)*) => { (count_tts!($($a)*) << 1) | 1 };
//     ($($a:tt $even:tt)*) => { count_tts!($($a)*) << 1 };
// }

macro_rules! decl_vector {{
    $(#[$type_meta:meta])*
    $vis:vis struct $Type:ident {$(
        $(#[$field_meta:meta])*
        $field:ident
    ),* $(,)?}
} => {
    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Eq)]
    $(#[$type_meta])*
    $vis struct $Type<T> {$(
        $(#[$field_meta])*
        $vis $field: T,
    )*}

    impl<T> $Type<T> {
        pub const fn new($($field: T),*) -> Self {
            Self { $($field),* }
        }
    }
}}

decl_vector! {
    /// A 2-dimensional vector.
    ///
    /// | coordinate space | conventional `x` | conventional `y` |
    /// | ---------------- | ---------------- | ---------------- |
    /// | side-on          | forward (right)  | up               |
    /// | top-down         | east (right)     | north (up)       |
    /// | texture space    | right            | down             |
    pub struct Vector2 { x, y }
}

decl_vector! {
    /// A 3-dimensional vector.
    ///
    /// | coordinate space | conventional `x` | conventional `y` | conventional `z` |
    /// | ---------------- | ---------------- | ---------------- | ---------------- |
    /// | local space      | forward          | left             | up               |
    /// | world space      | north            | west             | up               |
    /// | view space       | right            | up               | closer           |
    /// | clip space       | right ∈ [-1, 1]  | up ∈ [-1, 1]     | further ∈ [0, 1] |
    ///
    /// <div class="warning">Note that while Pyrr uses a right-handed coordinate system,
    /// rendering ends up left-handed due to the use of a reverse-z depth. However, this
    /// should only ever matter when doing clip-space work in shaders.</div>
    pub struct Vector3 { x, y, z }
}

macro_rules! impl_vector {{
    $(trait $Guest:ident = Guest;)?
    $(const $N:ident: usize;)?
    type $Vector:ident;
    $($items:item)*
} => {
    const _: () = {
        $(const $N: usize = 2;)?
        $(use crate::ffi::vec2f::Guest as $Guest;)?
        type $Vector<T> = Vector2<T>;
        $($items)*
    };
    const _: () = {
        $(const $N: usize = 3;)?
        $(use crate::ffi::vec3f::Guest as $Guest;)?
        type $Vector<T> = Vector3<T>;
        $($items)*
    };
}}

#[allow(unsafe_code)]
const unsafe fn transmute_prefix<Src, Dst>(src: Src) -> Dst {
    union Union<Src, Dst> {
        src: ManuallyDrop<Src>,
        dst: ManuallyDrop<Dst>,
    }

    let src = ManuallyDrop::new(src);
    ManuallyDrop::into_inner(Union { src }.dst)
}

impl_vector!(
    const N: usize;
    type Vector;

    impl<T> Vector<T> {
        #[allow(unsafe_code)]
        pub const fn as_array(&self) -> &[T; N] {
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { &*(self as *const Self).cast() }
        }

        #[allow(unsafe_code)]
        pub fn as_mut_array(&mut self) -> &mut [T; N] {
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { &mut *(self as *mut Self).cast() }
        }

        #[allow(unsafe_code)]
        pub const fn into_array(self) -> [T; N] {
            assert!(size_of::<Vector<T>>() == size_of::<[T; N]>());
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { transmute_prefix(self) }
        }

        pub const fn as_slice(&self) -> &[T] {
            self.as_array().as_slice()
        }

        pub fn as_mut_slice(&mut self) -> &mut [T] {
            self.as_mut_array()
        }

        pub const fn into_glm(self) -> glm::TVec<T, N> {
            glm::TVec::from_array_storage(na::base::ArrayStorage([self.into_array(); 1]))
        }
    }

    impl<T> From<[T; N]> for Vector<T> {
        #[allow(unsafe_code)]
        fn from(v: [T; N]) -> Self {
            assert_eq!(size_of::<Vector<T>>(), size_of::<[T; N]>());
            // SAFETY: Vector is repr(C) with N fields of type T
            unsafe { transmute_copy(&ManuallyDrop::new(v)) }
        }
    }

    impl<T> From<Vector<T>> for [T; N] {
        fn from(v: Vector<T>) -> Self {
            v.into_array()
        }
    }

    impl<'a, T> From<&'a Vector<T>> for &'a [T; N] {
        fn from(v: &'a Vector<T>) -> Self {
            v.as_array()
        }
    }

    impl<'a, T> From<&'a mut Vector<T>> for &'a mut [T; N] {
        fn from(v: &'a mut Vector<T>) -> Self {
            v.as_mut_array()
        }
    }

    impl<T> AsRef<[T; N]> for Vector<T> {
        fn as_ref(&self) -> &[T; N] {
            self.as_array()
        }
    }

    impl<T> AsMut<[T; N]> for Vector<T> {
        fn as_mut(&mut self) -> &mut [T; N] {
            self.as_mut_array()
        }
    }

    impl<T> From<Vector<T>> for glm::TVec<T, N> {
        fn from(v: Vector<T>) -> Self {
            v.into_glm()
        }
    }

    impl<T> From<glm::TVec<T, N>> for Vector<T> {
        fn from(v: glm::TVec<T, N>) -> Self {
            let (a,) = v.data.0.into();
            a.into()
        }
    }

    impl<T: glm::Scalar> simba::scalar::SubsetOf<glm::TVec<T, N>> for Vector<T> {
        fn to_superset(&self) -> glm::TVec<T, N> {
            self.clone().into()
        }

        fn is_in_subset(_: &glm::TVec<T, N>) -> bool {
            true
        }

        fn from_superset_unchecked(v: &glm::TVec<T, N>) -> Self {
            v.clone().into()
        }
    }

    impl<T: glm::Scalar> simba::scalar::SubsetOf<Vector<T>> for glm::TVec<T, N> {
        fn to_superset(&self) -> Vector<T> {
            self.clone().into()
        }

        fn is_in_subset(_: &Vector<T>) -> bool {
            true
        }

        fn from_superset_unchecked(v: &Vector<T>) -> Self {
            v.clone().into()
        }
    }

    impl<T: Scalar> Neg for Vector<T> {
        type Output = Self;
        fn neg(self) -> Self {
            <[T; N]>::from(self).map(Neg::neg).into()
        }
    }

    impl<T: Scalar> AddAssign for Vector<T> {
        fn add_assign(&mut self, rhs: Self) {
            for (this, rhs) in self.as_mut_slice().iter_mut().zip(<[T; N]>::from(rhs)) {
                *this += rhs;
            }
        }
    }

    impl<T: Scalar> Add for Vector<T> {
        type Output = Self;
        fn add(mut self, rhs: Self) -> Self {
            self += rhs;
            self
        }
    }

    impl<T: Scalar> SubAssign for Vector<T> {
        fn sub_assign(&mut self, rhs: Self) {
            for (this, rhs) in self.as_mut_slice().iter_mut().zip(<[T; N]>::from(rhs)) {
                *this -= rhs;
            }
        }
    }

    impl<T: Scalar> Sub for Vector<T> {
        type Output = Self;
        fn sub(mut self, rhs: Self) -> Self {
            self -= rhs;
            self
        }
    }
);

impl<T> mint::IntoMint for Vector2<T> {
    type MintType = mint::Vector2<T>;
}

impl<T> From<mint::Vector2<T>> for Vector2<T> {
    fn from(v: mint::Vector2<T>) -> Self {
        Self::new(v.x, v.y)
    }
}

impl<T> From<Vector2<T>> for mint::Vector2<T> {
    fn from(v: Vector2<T>) -> Self {
        let Vector2 { x, y } = v;
        Self { x, y }
    }
}

impl<T> mint::IntoMint for Vector3<T> {
    type MintType = mint::Vector3<T>;
}

impl<T> From<mint::Vector3<T>> for Vector3<T> {
    fn from(v: mint::Vector3<T>) -> Self {
        Self::new(v.x, v.y, v.z)
    }
}

impl<T> From<Vector3<T>> for mint::Vector3<T> {
    fn from(v: Vector3<T>) -> Self {
        let Vector3 { x, y, z } = v;
        Self { x, y, z }
    }
}

impl_vector!(
    const N: usize;
    type Vector;

    impl<T: Scalar> Vector<T> {
        pub fn splat(v: T) -> Self {
            glm::TVec::<T, N>::repeat(v).into()
        }

        pub fn abs(self) -> Self {
            glm::TVec::<T, N>::from(self).abs().into()
        }

        pub fn signum(self) -> Self {
            glm::sign(&glm::TVec::<T, N>::from(self)).into()
        }

        pub fn min(self, rhs: Self) -> Self {
            glm::min2(
                &glm::TVec::<T, N>::from(self),
                &glm::TVec::<T, N>::from(rhs),
            )
            .into()
        }

        pub fn max(self, rhs: Self) -> Self {
            glm::max2(
                &glm::TVec::<T, N>::from(self),
                &glm::TVec::<T, N>::from(rhs),
            )
            .into()
        }

        pub fn clamp(self, min: Self, max: Self) -> Self {
            glm::clamp_vec(
                &glm::TVec::<T, N>::from(self),
                &glm::TVec::<T, N>::from(min),
                &glm::TVec::<T, N>::from(max),
            )
            .into()
        }
    }
);

impl_vector!(
    const N: usize;
    type Vector;

    // rust-lang/rustfmt#6036
    impl Vector<f32> {
        pub fn ceil(self) -> Self {
            glm::ceil(&glm::TVec::<f32, N>::from(self)).into()
        }

        pub fn floor(self) -> Self {
            glm::floor(&glm::TVec::<f32, N>::from(self)).into()
        }

        pub fn trunc(self) -> Self {
            glm::trunc(&glm::TVec::<f32, N>::from(self)).into()
        }

        pub fn fract(self) -> Self {
            glm::fract(&glm::TVec::<f32, N>::from(self)).into()
        }

        pub fn round(self) -> Self {
            glm::round(&glm::TVec::<f32, N>::from(self)).into()
        }

        pub fn copysign(self, sign: Self) -> Self {
            glm::TVec::<f32, N>::from(self)
                .zip_map(&glm::TVec::<f32, N>::from(sign), na::RealField::copysign)
                .into()
        }

        pub fn powf(self, pow: f32) -> Self {
            glm::TVec::<f32, N>::from(self)
                .map(|this| na::ComplexField::powf(this, pow))
                .into()
        }

        pub fn powi(self, pow: i32) -> Self {
            glm::TVec::<f32, N>::from(self)
                .map(|this| na::ComplexField::powi(this, pow))
                .into()
        }

        pub fn recip(self) -> Self {
            glm::TVec::<f32, N>::from(self)
                .map(na::ComplexField::recip)
                .into()
        }

        pub fn len(self) -> f32 {
            glm::TVec::<f32, N>::from(self).magnitude()
        }

        pub fn len_recip(self) -> f32 {
            na::ComplexField::recip(Self::len(self))
        }

        pub fn len_sq(self) -> f32 {
            glm::TVec::<f32, N>::from(self).magnitude_squared()
        }

        pub fn len_sq_recip(self) -> f32 {
            na::ComplexField::recip(Self::len_sq(self))
        }

        pub fn normalize(self) -> Self {
            match Self::normalize_or_wild(self) {
                it if Self::is_normalized(it) => it,
                _ => cold_panic!("attempted to normalize vector with length zero"),
            }
        }

        pub fn normalize_or_zero(self) -> Self {
            match Self::normalize_or_wild(self) {
                it if Self::is_normalized(it) => it,
                _ => glm::TVec::<f32, N>::zeros().into(),
            }
        }

        pub fn normalize_or_wild(self) -> Self {
            glm::TVec::<f32, N>::from(self).normalize().into()
        }

        pub fn is_normalized(self) -> bool {
            const X1P_20: f32 = 9.536_743e-7; // 2^-20 ≅ 10^-6
            #[allow(clippy::cast_precision_loss)]
            const _: () = assert!(X1P_20 == (1.0 / (1 << 20) as f32));
            Self::is_normalized_enough(self, X1P_20)
        }

        pub fn is_normalized_enough(self, epsilon: f32) -> bool {
            glm::is_normalized(&glm::TVec::<f32, N>::from(self), epsilon)
        }

        pub fn project_onto(self, onto: Self) -> Self {
            let onto = glm::TVec::<f32, N>::from(onto);
            glm::TVec::<f32, N>::from(self).dot(&onto).mul(onto).into()
        }

        pub fn reject_from(self, onto: Self) -> Self {
            Self::sub(self, Self::project_onto(self, onto))
        }

        pub fn is_nan(self) -> bool {
            glm::any(&glm::TVec::<f32, N>::from(self).map(f32::is_nan))
        }

        pub fn is_finite(self) -> bool {
            glm::all(
                &glm::TVec::<f32, N>::from(self).map(|this| na::ComplexField::is_finite(&this)),
            )
        }
    }
);

impl_vector!(
    trait Guest = Guest;
    type Vector;

    // rust-lang/rustfmt#6036
    impl Guest for Vector<f32> {
        fn static_splat(v: f32) -> Self {
            Self::splat(v)
        }

        fn method_abs(this: Self) -> Self {
            this.abs()
        }

        fn method_neg(this: Self) -> Self {
            this.neg()
        }

        fn method_ceil(this: Self) -> Self {
            this.ceil()
        }

        fn method_floor(this: Self) -> Self {
            this.floor()
        }

        fn method_trunc(this: Self) -> Self {
            this.trunc()
        }

        fn method_fract(this: Self) -> Self {
            this.fract()
        }

        fn method_round(this: Self) -> Self {
            this.round()
        }

        fn method_signum(this: Self) -> Self {
            this.signum()
        }

        fn method_add(this: Self, rhs: Self) -> Self {
            this.add(rhs)
        }

        fn method_sub(this: Self, rhs: Self) -> Self {
            this.sub(rhs)
        }

        fn method_min(this: Self, rhs: Self) -> Self {
            this.min(rhs)
        }

        fn method_max(this: Self, rhs: Self) -> Self {
            this.max(rhs)
        }

        fn method_copysign(this: Self, sign: Self) -> Self {
            this.copysign(sign)
        }

        fn method_eq(this: Self, rhs: Self) -> bool {
            this.eq(&rhs)
        }

        fn method_ne(this: Self, rhs: Self) -> bool {
            this.ne(&rhs)
        }

        fn method_clamp(this: Self, min: Self, max: Self) -> Self {
            this.clamp(min, max)
        }

        fn method_powf(this: Self, pow: f32) -> Self {
            this.powf(pow)
        }

        fn method_powi(this: Self, pow: i32) -> Self {
            this.powi(pow)
        }

        fn method_recip(this: Self) -> Self {
            this.recip()
        }

        fn method_len(this: Self) -> f32 {
            this.len()
        }

        fn method_len_recip(this: Self) -> f32 {
            this.len_recip()
        }

        fn method_len_sq(this: Self) -> f32 {
            this.len_sq()
        }

        fn method_len_sq_recip(this: Self) -> f32 {
            this.len_sq_recip()
        }

        fn method_normalize(this: Self) -> Self {
            this.normalize()
        }

        fn method_normalize_or_zero(this: Self) -> Self {
            this.normalize_or_zero()
        }

        fn method_normalize_or_wild(this: Self) -> Self {
            this.normalize_or_wild()
        }

        fn method_is_normalized(this: Self) -> bool {
            this.is_normalized()
        }

        fn method_is_normalized_enough(this: Self, epsilon: f32) -> bool {
            this.is_normalized_enough(epsilon)
        }

        fn method_project_onto(this: Self, onto: Self) -> Self {
            this.project_onto(onto)
        }

        fn method_reject_from(this: Self, onto: Self) -> Self {
            this.reject_from(onto)
        }

        fn method_is_nan(this: Self) -> bool {
            this.is_nan()
        }

        fn method_is_finite(this: Self) -> bool {
            this.is_finite()
        }
    }
);

/// A 2-dimensional vector.
pub type Vec2<T> = Vector2<T>;
/// A 3-dimensional vector.
pub type Vec3<T> = Vector3<T>;

/// A 2-dimensional vector of `f32`s.
pub type Vec2f = Vector2<f32>;
/// A 3-dimensional vector of `f32`s.
pub type Vec3f = Vector3<f32>;

/// A 2-dimensional vector of `i32`s.
pub type Vec2i = Vector2<i32>;
/// A 3-dimensional vector of `i32`s.
pub type Vec3i = Vector3<i32>;

/// A 2-dimensional vector of `bool`s.
pub type Vec2b = Vector2<bool>;
/// A 3-dimensional vector of `bool`s.
pub type Vec3b = Vector3<bool>;

/// A number, either `f32` or `i32`.
pub trait Scalar: glm::Number {}

impl Scalar for f32 {}
impl Scalar for i32 {}
