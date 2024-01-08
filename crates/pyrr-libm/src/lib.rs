wit_bindgen::generate!({
    world: "exports",
    exports: {
        "pyrr:libm/f32": Exports,
        "pyrr:libm/f64": Exports,
    },
});

pub struct Exports;

use libm::{lgammaf_r as lgamma_rf, *};

macro_rules! export_libm {
    {
        $(fn $fn:ident($($arg:ident: $Arg:ty),*) -> $Ret:ty;)*
    } => {
        const _: () = {
            type Float = f32;
            impl exports::pyrr::libm::f32::Guest for Exports {
                $(
                    fn $fn($($arg: $Arg),*) -> $Ret {
                        paste::paste! { [<$fn f>]($($arg),*) }
                    }
                )*
            }
        };
        const _: () = {
            type Float = f64;
            impl exports::pyrr::libm::f64::Guest for Exports {
                $(
                    fn $fn($($arg: $Arg),*) -> $Ret {
                        $fn($($arg),*)
                    }
                )*
            }
        };
    };
}

export_libm! {
    fn acos(x: Float) -> Float;
    fn acosh(x: Float) -> Float;
    fn asin(x: Float) -> Float;
    fn atan(x: Float) -> Float;
    fn atan2(y: Float, x: Float) -> Float;
    fn atanh(x: Float) -> Float;
    fn cbrt(x: Float) -> Float;
    fn ceil(x: Float) -> Float;
    fn copysign(x: Float, y: Float) -> Float;
    fn cos(x: Float) -> Float;
    fn cosh(x: Float) -> Float;
    fn erf(x: Float) -> Float;
    fn erfc(x: Float) -> Float;
    fn exp(x: Float) -> Float;
    fn exp2(x: Float) -> Float;
    fn exp10(x: Float) -> Float;
    fn expm1(x: Float) -> Float;
    fn fabs(x: Float) -> Float;
    fn fdim(x: Float, y: Float) -> Float;
    fn floor(x: Float) -> Float;
    fn fma(x: Float, y: Float, z: Float) -> Float;
    fn fmax(x: Float, y: Float) -> Float;
    fn fmin(x: Float, y: Float) -> Float;
    fn fmod(x: Float, y: Float) -> Float;
    fn frexp(x: Float) -> (Float, i32);
    fn hypot(x: Float, y: Float) -> Float;
    fn ilogb(x: Float) -> i32;
    fn j0(x: Float) -> Float;
    fn j1(x: Float) -> Float;
    fn jn(n: i32, x: Float) -> Float;
    fn ldexp(x: Float, n: i32) -> Float;
    fn lgamma(x: Float) -> Float;
    fn lgamma_r(x: Float) -> (Float, i32);
    fn log(x: Float) -> Float;
    fn log1p(x: Float) -> Float;
    fn log2(x: Float) -> Float;
    fn log10(x: Float) -> Float;
    fn modf(x: Float) -> (Float, Float);
    fn nextafter(x: Float, y: Float) -> Float;
    fn pow(x: Float, y: Float) -> Float;
    fn remainder(x: Float, y: Float) -> Float;
    fn remquo(x: Float, y: Float) -> (Float, i32);
    fn rint(x: Float) -> Float;
    fn round(x: Float) -> Float;
    fn scalbn(x: Float, n: i32) -> Float;
    fn sin(x: Float) -> Float;
    fn sincos(x: Float) -> (Float, Float);
    fn sinh(x: Float) -> Float;
    fn sqrt(x: Float) -> Float;
    fn tan(x: Float) -> Float;
    fn tanh(x: Float) -> Float;
    fn tgamma(x: Float) -> Float;
    fn trunc(x: Float) -> Float;
    fn y0(x: Float) -> Float;
    fn y1(x: Float) -> Float;
    fn yn(n: i32, x: Float) -> Float;
}
