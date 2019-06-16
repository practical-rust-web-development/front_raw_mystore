use std::sync::Arc;
use wasm_bindgen::JsValue ;
use crate::app::App;
use crate::components::component::Component;

#[derive(Clone)]
pub struct Register {
    url: String,
    app: Arc<App>
}

impl Register {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Register { url, app }
    }
}

impl Component for Register {
    fn app(&self) -> Arc<App> { self.app.clone() }

    fn url(&self) -> String { self.url.clone() }

    fn load_components(&self) -> Result<(), JsValue> {

        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Register an User"));

        self.app.div.append_child(&h2_title)?;

        Ok(())
    }
}