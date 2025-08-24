pub mod app;
pub mod subpage;
pub mod anotherpage;
pub mod homepage;
pub mod impressum;
pub mod common;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    #[allow(unused)]
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
