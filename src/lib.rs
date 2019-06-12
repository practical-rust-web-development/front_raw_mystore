use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
mod fetch;
mod router;

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
    let history = window.history().expect("no history");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let app = document.get_element_by_id("app").expect("A div with id app is required");

    let button = document.get_element_by_id("go_to_register").unwrap();
    let button_et: web_sys::EventTarget = button.into();

    let show_register = Closure::wrap(Box::new(move || {
        let router = router::Router(Some(&app));
        router.go_to("register", "");
    }) as Box<dyn FnMut()>);

    button_et.add_event_listener_with_callback("click", show_register.as_ref().unchecked_ref());

    window.set_onpopstate(Some(show_register.as_ref().unchecked_ref()));

    show_register.forget();

    Ok(())
}