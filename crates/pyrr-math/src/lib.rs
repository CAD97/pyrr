mod wit {
    pub struct Exports;
    wit_bindgen::generate!({
        world: "exports",
        exports: {
            "pyrr:math/libm": Exports,
            "pyrr:math/vec2f": Exports,
        },
        with: {
            "pyrr:math/types": super::types,
        }
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
mod types;
mod vec;

pub use self::{libm::*, types::*, vec::*};
