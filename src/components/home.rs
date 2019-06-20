use wasm_bindgen::prelude::*;
use std::sync::Arc;
use wasm_bindgen::{ JsValue, JsCast };
use web_sys::EventTarget;
use crate::app::App;
use crate::components::{ register, login };
use crate::components::component::{ Component, FlashMessage };

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

    fn load_components(&self, data: &JsValue) -> Result<(), JsValue> {

        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("My Store"));

        self.app.div.append_child(&h2_title)?;

        let button = self.app.document.create_element("button")?;
        button.set_id("go_to_register");
        button.set_text_content(Some("Register"));
        button.set_class_name("btn btn-primary");

        let login_btn = self.app.document.create_element("button")?;
        login_btn.set_id("go_to_login");
        login_btn.set_text_content(Some("Login"));
        login_btn.set_class_name("btn btn-primary");

        self.app.div.append_child(&button)?;
        self.app.div.append_child(&login_btn)?;

        let result_flash_message = data.into_serde::<FlashMessage>();

        if let Ok(flash_message) = result_flash_message {
            let flash_message_div = self.app.document.create_element("div")
                .expect("Creating alert not possible");
            flash_message_div.set_class_name("alert alert-info");
            flash_message_div.set_text_content(Some(&flash_message.message));
            self.app.div.append_child(&flash_message_div);
        }

        let app_for_register = self.app.clone();
        
        let button_et: EventTarget = button.into();

        let handler = 
            Closure::wrap(Box::new(move || {
                register::Register::new(
                    "register".to_string(),
                     app_for_register.clone()).render(&JsValue::from_str(""));
            }) as Box<dyn FnMut()>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();

        let button_login_et: EventTarget = login_btn.into();
        let app_for_login = self.app.clone();
        let handler = 
            Closure::wrap(Box::new(move || {
                login::Login::new(
                    "login".to_string(),
                     app_for_login.clone()).render(&JsValue::from_str(""));
            }) as Box<dyn FnMut()>);

        button_login_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();
        Ok(())
    }

}