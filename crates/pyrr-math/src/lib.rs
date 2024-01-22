mod wit {
    pub struct Exports;
    wit_bindgen::generate!({
        world: "exports",
        exports: {
            "pyrr:math/libm": Exports,
            "pyrr:math/vec2f": super::Vec2f,
            "pyrr:math/vec3f": super::Vec3f,
        },
        with: {
            "pyrr:math/vec": super::vec,
        },
    });
}

pub mod ffi {
    pub use crate::wit::exports::pyrr::math::*;
    pub use crate::wit::Exports;
}

macro_rules! cold_panic {
    ($($($args:tt)+)?) => {{ #[allow(clippy::redundant_closure_call)] {
        (|| -> ! { panic!($($($args)+)?) })()
    }}};
}

mod libm;
mod vec;

pub use self::{libm::*, vec::*};
