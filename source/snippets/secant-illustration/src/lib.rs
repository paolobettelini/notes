use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

#[macro_use]
extern crate lazy_static;

mod secant;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        secant::run_app().await;
    });
}
