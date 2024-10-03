use gfx_toybox::{run, print_adapters};

fn init_logger() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder()
            .filter(Some(module_path!()), log::LevelFilter::Info)
            .parse_default_env()
            .init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }
}

fn main() {
    init_logger();

    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(print_adapters());

    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(print_adapters());

    run();
}
