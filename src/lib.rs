use wasm_bindgen::prelude::*;
use web_sys::window;
use std::sync::Arc;
use wasm_bindgen::JsCast;

mod fetch;
mod router;
mod app;
mod components;

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