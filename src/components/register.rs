use std::sync::Arc;
use wasm_bindgen::JsValue ;
use crate::app::App;

pub struct Register {
    url: String,
    app: Arc<App>
}

impl Register {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Register { url, app }
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.app.go_to(&self.url, "")?;
        self.load_components()
    }

    fn load_components(&self) -> Result<(), JsValue> {

        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Register an User"));

        self.app.div.append_child(&h2_title)?;

        Ok(())
    }

}