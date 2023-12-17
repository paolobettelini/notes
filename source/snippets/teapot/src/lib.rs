use async_std::task::block_on;
use wasm_bindgen::prelude::wasm_bindgen;

pub(crate) mod data;
mod teapot;

// web app entry_point
#[wasm_bindgen]
pub async fn main_web() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    block_on(async {
        teapot::run_app().await;
    });
}
