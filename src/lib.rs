// src/lib.rs
mod api;
mod app;
mod app_state;
mod components;
mod pages;
mod route;
mod types;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::Renderer::<app::App>::new().render();
}
