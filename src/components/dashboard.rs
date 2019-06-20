use std::sync::Arc;
use wasm_bindgen::JsValue;
use crate::app::App;
use crate::components::component::{ Component };

#[derive(Clone)]
pub struct Dashboard {
    url: String,
    app: Arc<App>
}

impl Dashboard {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Dashboard { url, app }
    }
}

impl Component for Dashboard {
    fn app(&self) -> Arc<App> { self.app.clone() }

    fn url(&self) -> String { self.url.clone() }

    fn load_components(&self, data: &JsValue) -> Result<(), JsValue> {

        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Welcome to Dashboard"));

        self.app.div.append_child(&h2_title)?;
        Ok(())
    }

}
