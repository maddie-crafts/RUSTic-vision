use leptos::*;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;

pub mod app;
pub mod ascii;
pub mod cli;
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App /> });
}