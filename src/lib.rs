pub mod app;
pub mod homepage;
pub mod photos;
pub mod common;
pub mod vita;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused)]
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
