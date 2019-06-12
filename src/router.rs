use wasm_bindgen::prelude::*;
use web_sys::Element;
use wasm_bindgen::convert::{FromWasmAbi, Stack};
use wasm_bindgen::describe::WasmDescribe;

pub struct Router<'a>(pub Option<&'a Element>);

impl<'a> FromWasmAbi for Router<'a> {
    type Abi = <Element as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
        Router(None)
    }
}

impl<'a> WasmDescribe for Router<'a> {
    fn describe() { Element::describe(); }
}

impl<'a> Router<'a> {
    pub fn go_to(&self, url: &str, state: &str) {
        let (history, location) = Self::history_and_location();

        let data = JsValue::from_str(state);

        history.push_state_with_url(&data, 
            url, Some(&format!("{}/{}", location.origin().unwrap(), url)));
        
        let template = format!("<object type=\"text/html\" data=\"public/templates/{}.html\" ></object>", url);
        web_sys::console::log_1(&JsValue::from_str(&template));
        self.0.map(|app| app.set_inner_html(&template));
    }

    fn history_and_location() -> (web_sys::History, web_sys::Location) {
        let window = web_sys::window().expect("no global `window` exists");
        let history = window.history().expect("no history");
        let document = window.document().expect("should have a document on window");
        let location = document.location().unwrap();

        (history, location)
    }
}
