use std::sync::Arc;
use wasm_bindgen::JsValue;
use crate::app::App;

pub trait Component {
    fn load_components(&self) -> Result<(), JsValue>;
    fn app(&self) -> Arc<App>;
    fn url(&self) -> String;
    fn render(&self) -> Result<(), JsValue> {
        self.app().div.set_inner_html("");
        self.load_components()?;
        self.app().go_to(&self.url(), "")
    }
}