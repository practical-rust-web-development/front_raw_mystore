use wasm_bindgen::prelude::*;
use web_sys::Element;
use wasm_bindgen::convert::{FromWasmAbi, Stack};
use wasm_bindgen::describe::WasmDescribe;

pub struct Router(pub Element);

impl FromWasmAbi for Router {
    type Abi = <Element as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
        Router(Element::from_abi(js, extra))
    }
}

impl WasmDescribe for Router {
    fn describe() { Element::describe(); }
}

impl Router {
    pub fn go_to(&self, url: &str, state: &str) {
        let (history, location) = Self::history_and_location();

        let data = JsValue::from_str(state);

        // history.push_state_with_url(&data, 
        //     url, Some(&format!("{}/{}", location.origin().unwrap(), url)));
        
        let template = format!("<object type=\"text/html\" data=\"public/templates/{}.html\" ></object>", url);
        web_sys::console::log_1(&JsValue::from_str(&template));
        self.0.set_inner_html(&template);
    }

    fn history_and_location() -> (web_sys::History, web_sys::Location) {
        let window = web_sys::window().expect("no global `window` exists");
        let history = window.history().expect("no history");
        let document = window.document().expect("should have a document on window");
        let location = document.location().unwrap();

        (history, location)
    }
}
