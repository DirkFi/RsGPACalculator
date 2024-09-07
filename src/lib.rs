// src/lib.rs
mod pages;
use pages::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Home>::new().render();
    // App::<Hello>::new().mount_to_body();
}
