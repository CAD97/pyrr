use pyrr::math::vec::Vec2f;
use std::path::Path;
use wasmtime::{
    component::{bindgen, Component, Linker},
    Config, Engine, Store,
};

bindgen!({
    path: "../demo-component/wit",
});

#[derive(Default)]
struct MyState {}

fn main() -> wasmtime::Result<()> {
    let out_dir = Path::new(env!("OUT_DIR"));
    let demo_wasm = out_dir.join("../../../../wasm-component/pyrr-demo.wasm");
    let math_wasm = out_dir.join("../../../../wasm-component/pyrr-math.wasm");

    let engine = Engine::new(Config::new().wasm_component_model(true))?;
    let mut linker = Linker::new(&engine);
    let mut store = Store::new(&engine, MyState::default());

    {
        // pre-composed wasm blob: wasm-tools compose pyrr-demo.wasm -d pyrr-math.wasm -o test.wasm
        let test = Component::from_file(&engine, "test.wasm")?;
        let (bindings, _) = Exports::instantiate(&mut store, &test, &linker)?;
        dbg!(bindings.interface0.call_run(
            &mut store,
            Vec2f { x: 1., y: 2. },
            Vec2f { x: 3., y: 4. }
        )?);
    }

    let math = Component::from_file(&engine, math_wasm)?;
    let demo = Component::from_file(&engine, demo_wasm)?;

    let math = linker.instantiate(&mut store, &math)?;
    // let demo = linker.instantiate(&mut store, &demo)?;
    // let bindings = Exports::new(&mut store, &demo)?;

    _ = (&mut linker, demo, math);

    Ok(())
}
