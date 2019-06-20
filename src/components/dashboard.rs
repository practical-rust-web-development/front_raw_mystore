use std::sync::Arc;
use wasm_bindgen::JsValue;
use crate::app::App;
use crate::components::component::{ Component, FlashMessage };

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

        let result_flash_message = data.into_serde::<FlashMessage>();

        if let Ok(flash_message) = result_flash_message {
            let flash_message_div = self.app.document.create_element("div")
                .expect("Creating alert not possible");
            flash_message_div.set_class_name("alert alert-info");
            flash_message_div.set_text_content(Some(&flash_message.message));
            self.app.div.append_child(&flash_message_div);
        }

        Ok(())
    }

}
