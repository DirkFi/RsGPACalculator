// src/lib.rs
mod api;
mod pages;
mod types;
use pages::Home;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<Home>::new().render();
}
