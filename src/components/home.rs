use wasm_bindgen::prelude::*;
use std::sync::Arc;
use wasm_bindgen::{ JsValue, JsCast };
use web_sys::EventTarget;
use crate::app::App;
use crate::components::register;
use crate::components::component::Component;

#[derive(Clone)]
pub struct Home {
    url: String,
    app: Arc<App>
}

impl Home {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Home { url, app }
    }
}

impl Component for Home {
    fn app(&self) -> Arc<App> { self.app.clone() }

    fn url(&self) -> String { self.url.clone() }

    fn load_components(&self) -> Result<(), JsValue> {

        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("My Store"));

        self.app.div.append_child(&h2_title)?;

        let button = self.app.document.create_element("button")?;
        button.set_id("go_to_register");
        button.set_text_content(Some("Register"));

        self.app.div.append_child(&button)?;

        let button_et: EventTarget = button.into();

        let app_closure = self.app.clone();
        
        let handler = 
            Closure::wrap(Box::new(move || {
                register::Register::new("register".to_string(), app_closure.clone()).render();
            }) as Box<dyn FnMut()>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();
        Ok(())
    }

}