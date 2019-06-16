use wasm_bindgen::prelude::*;
use web_sys::{ EventTarget, window };
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasm_bindgen::JsCast;
use crate::app::App;

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

    let window = window().expect("no global `window` exists");
    let location = window.clone().location();
    let pathname = location.pathname().expect("no path");
    components::routes::Routes::new(application.clone()).go_to(pathname.clone());

    let handler = 
        Closure::wrap(Box::new(move || {
            application.clone().div.set_inner_html("");
            let pathname = location.pathname().expect("no path");
            components::routes::Routes::new(application.clone()).load_components(pathname.clone());
        }) as Box<dyn FnMut()>);

    window.set_onpopstate(Some(handler.as_ref().unchecked_ref()));
    handler.forget();
    Ok(())
}