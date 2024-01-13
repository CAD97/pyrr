macro_rules! forward_libm {
    {
        $(fn $fn:ident($($arg:ident: $Arg:ty),*) -> $Ret:ty;)*
    } => {
        $(fn $fn($($arg: $Arg),*) -> $Ret {
            libm::$fn($($arg),*)
        })*
    };
}

// wasm builtin: ceil, ceilf, fabs, fabsf, floor, floorf, sqrt, sqrtf, trunc, truncf
// also benefit: rem_pio2_large, round, roundf

pub use libm::*;

// MAYBE: patch libm crate to use some of these less precise impls?
impl crate::ffi::libm::Guest for crate::ffi::Exports {
    forward_libm! {
        fn acosf(x: f32) -> f32;
        fn acoshf(x: f32) -> f32;
        fn asinf(x: f32) -> f32;
        fn atanf(x: f32) -> f32;
        fn atan2f(y: f32, x: f32) -> f32;
        fn atanhf(x: f32) -> f32;
        fn cbrtf(x: f32) -> f32;
        fn ceilf(x: f32) -> f32;
        fn copysignf(x: f32, y: f32) -> f32;
        fn cosf(x: f32) -> f32;
        fn coshf(x: f32) -> f32;
        fn erff(x: f32) -> f32; // rare
        fn erfcf(x: f32) -> f32; // rare
        fn expf(x: f32) -> f32;
        fn exp2f(x: f32) -> f32;
        fn exp10f(x: f32) -> f32; // rare
        fn expm1f(x: f32) -> f32;
        fn fabsf(x: f32) -> f32;
        fn fdimf(x: f32, y: f32) -> f32; // rare
        fn floorf(x: f32) -> f32;
        fn fmaf(x: f32, y: f32, z: f32) -> f32;
        fn fmaxf(x: f32, y: f32) -> f32; // rare
        fn fminf(x: f32, y: f32) -> f32; // rare
        fn fmodf(x: f32, y: f32) -> f32; // rare
        fn frexpf(x: f32) -> (f32, i32); // rare
        fn hypotf(x: f32, y: f32) -> f32;
        fn ilogbf(x: f32) -> i32; // rare
        fn j0f(x: f32) -> f32; // rare
        fn j1f(x: f32) -> f32; // rare
        fn jnf(n: i32, x: f32) -> f32; // rare
        fn ldexpf(x: f32, n: i32) -> f32; // rare
        fn lgammaf(x: f32) -> f32; // rare
        fn lgammaf_r(x: f32) -> (f32, i32); // rare
        fn logf(x: f32) -> f32;
        fn log1pf(x: f32) -> f32;
        fn log2f(x: f32) -> f32;
        fn log10f(x: f32) -> f32;
        fn modff(x: f32) -> (f32, f32); // rare
        fn nextafterf(x: f32, y: f32) -> f32; // rare
        fn powf(x: f32, y: f32) -> f32;
        fn remainderf(x: f32, y: f32) -> f32; // rare
        fn remquof(x: f32, y: f32) -> (f32, i32); // rare
        fn rintf(x: f32) -> f32; // rare
        fn roundf(x: f32) -> f32;
        fn scalbnf(x: f32, n: i32) -> f32;
        fn sinf(x: f32) -> f32;
        fn sincosf(x: f32) -> (f32, f32);
        fn sinhf(x: f32) -> f32;
        fn sqrtf(x: f32) -> f32;
        fn tanf(x: f32) -> f32;
        fn tanhf(x: f32) -> f32;
        fn tgammaf(x: f32) -> f32; // rare
        fn truncf(x: f32) -> f32;
        fn y0f(x: f32) -> f32; // rare
        fn y1f(x: f32) -> f32; // rare
        fn ynf(n: i32, x: f32) -> f32; // rare

        fn acos(x: f64) -> f64;
        fn acosh(x: f64) -> f64;
        fn asin(x: f64) -> f64;
        fn atan(x: f64) -> f64;
        fn atan2(y: f64, x: f64) -> f64;
        fn atanh(x: f64) -> f64;
        fn cbrt(x: f64) -> f64;
        fn ceil(x: f64) -> f64;
        fn copysign(x: f64, y: f64) -> f64;
        fn cos(x: f64) -> f64;
        fn cosh(x: f64) -> f64;
        fn erf(x: f64) -> f64; // rare
        fn erfc(x: f64) -> f64; // rare
        fn exp(x: f64) -> f64;
        fn exp2(x: f64) -> f64;
        fn exp10(x: f64) -> f64; // rare
        fn expm1(x: f64) -> f64;
        fn fabs(x: f64) -> f64;
        fn fdim(x: f64, y: f64) -> f64; // rare
        fn floor(x: f64) -> f64;
        fn fma(x: f64, y: f64, z: f64) -> f64;
        fn fmax(x: f64, y: f64) -> f64; // rare
        fn fmin(x: f64, y: f64) -> f64; // rare
        fn fmod(x: f64, y: f64) -> f64; // rare
        fn frexp(x: f64) -> (f64, i32); // rare
        fn hypot(x: f64, y: f64) -> f64;
        fn ilogb(x: f64) -> i32; // rare
        fn j0(x: f64) -> f64; // rare
        fn j1(x: f64) -> f64; // rare
        fn jn(n: i32, x: f64) -> f64; // rare
        fn ldexp(x: f64, n: i32) -> f64; // rare
        fn lgamma(x: f64) -> f64; // rare
        fn lgamma_r(x: f64) -> (f64, i32); // rare
        fn log(x: f64) -> f64;
        fn log1p(x: f64) -> f64;
        fn log2(x: f64) -> f64;
        fn log10(x: f64) -> f64;
        fn modf(x: f64) -> (f64, f64); // rare
        fn nextafter(x: f64, y: f64) -> f64; // rare
        fn pow(x: f64, y: f64) -> f64;
        fn remainder(x: f64, y: f64) -> f64; // rare
        fn remquo(x: f64, y: f64) -> (f64, i32); // rare
        fn rint(x: f64) -> f64; // rare
        fn round(x: f64) -> f64;
        fn scalbn(x: f64, n: i32) -> f64;
        fn sin(x: f64) -> f64;
        fn sincos(x: f64) -> (f64, f64);
        fn sinh(x: f64) -> f64;
        fn sqrt(x: f64) -> f64;
        fn tan(x: f64) -> f64;
        fn tanh(x: f64) -> f64;
        fn tgamma(x: f64) -> f64; // rare
        fn trunc(x: f64) -> f64;
        fn y0(x: f64) -> f64; // rare
        fn y1(x: f64) -> f64; // rare
        fn yn(n: i32, x: f64) -> f64; // rare
    }
}
