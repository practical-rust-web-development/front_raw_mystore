use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::{ HtmlInputElement, Document };
use wasm_bindgen::JsCast;
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

pub struct InputComponent(pub Arc<Document>);

impl InputComponent {
    pub fn value_by_id(&self, id: &str) -> String {
        let element = self.0.get_element_by_id(id).expect(&format!("No {}", id));
        JsCast::dyn_ref::<HtmlInputElement>(&element).expect("Error casting input").value()
    }

}