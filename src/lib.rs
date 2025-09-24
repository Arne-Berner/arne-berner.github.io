pub mod app;
pub mod homepage;
pub mod impressum;
pub mod photos;
pub mod common;
pub mod vita;
pub mod audio;
pub mod showreel;
pub mod contact;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused)]
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
