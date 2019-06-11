use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
mod fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let app = document.get_element_by_id("app");

    Ok(())
}