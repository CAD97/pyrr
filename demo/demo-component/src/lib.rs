wit_bindgen::generate!({
    world: "exports",
    exports: {
        world: World,
        "pyrr:demo/component": Component,
    },
});

use pyrr::math::vec::Vec2f;

struct Component;

impl exports::pyrr::demo::component::Guest for Component {
    fn run(a: Vec2f, b: Vec2f) -> Vec2f {
        pyrr::math::vec2f_methods::method_add(a, b)
    }
}
