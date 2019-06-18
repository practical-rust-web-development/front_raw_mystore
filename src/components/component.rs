use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::{ HtmlInputElement, Document, Element };
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
    pub fn create_input(&self, id: &str, name: &str, placeholder: &str) 
        -> Result<Element, JsValue> {
            let div = self.0.create_element("div")?;
            div.set_class_name("from-group");
            let input_element = self.0.create_element("input")?;
            input_element.set_id(id);
            let input = JsCast::dyn_ref::<HtmlInputElement>(&input_element)
                .ok_or(JsValue::from_str("Error casting input"))?;
            input.set_placeholder(placeholder);
            input.set_class_name("form-control");
            input.set_name(name);
            div.append_child(input);
            Ok(div)
    }

    pub fn value_by_id(&self, id: &str) -> String {
        let element = self.0.get_element_by_id(id).expect(&format!("No {}", id));
        JsCast::dyn_ref::<HtmlInputElement>(&element).expect("Error casting input").value()
    }
}