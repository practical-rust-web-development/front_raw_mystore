use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

mod fetch;
mod router;
mod app;
mod components;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String
}

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let application = Arc::new(app::App::new());

    let home_component = components::home::Home::new("home".to_string(), application.clone());

    home_component.render()?;

    Ok(())
}