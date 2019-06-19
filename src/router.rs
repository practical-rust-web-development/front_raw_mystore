use wasm_bindgen::prelude::*;
use web_sys::{ History, Location };

pub struct Router {
    pub history: History,
    pub location: Location
}

impl Router {
    pub fn new() -> Self {
        let window = web_sys::window().expect("no global `window` exists");
        let history = window.history().expect("no history");
        let document = window.document().expect("should have a document on window");
        let location = document.location().unwrap();

        Router { history, location }
    }

    pub fn go_to(&self, url: &str, state: &JsValue) -> Result<(), JsValue> {
        self.history.push_state_with_url(state, 
            url, Some(&format!("{}/{}", self.location.origin().unwrap(), url)))
    }
}