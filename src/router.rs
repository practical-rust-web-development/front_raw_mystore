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

    pub fn go_to(&self, url: &str, state: &str) -> Result<(), JsValue> {
        let data = JsValue::from_str(state);

        self.history.push_state_with_url(&data, 
            url, Some(&format!("{}/{}", self.location.origin().unwrap(), url)))
    }
}