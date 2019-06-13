use wasm_bindgen::prelude::*;
use std::sync::Arc;
use wasm_bindgen::{ JsValue, JsCast };
use web_sys::EventTarget;
use crate::app::App;
use crate::components::register::Register;

pub struct Home {
    url: String,
    app: Arc<App>
}

impl Home {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Home { url, app }
    }

    pub fn render(&self) -> Result<(), JsValue> {
        self.app.go_to(&self.url, "")?;
        self.load_components()
    }

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
                app_closure.go_to("register", "");
                let component = Register::new("register".to_string(), app_closure.clone());
                component.render();
            }) as Box<dyn FnMut()>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();
        Ok(())
    }

}
