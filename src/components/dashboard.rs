use wasm_bindgen::prelude::*;
use std::sync::Arc;
use wasm_bindgen::{ JsValue, JsCast };
use web_sys::EventTarget;
use crate::app::App;
use crate::components::component::{ Component, FlashMessage };
use crate::fetch::delete_request;
use crate::components;

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

        let logout_btn = self.app.document.create_element("button")?;
        logout_btn.set_id("go_to_login");
        logout_btn.set_text_content(Some("Logout"));
        logout_btn.set_class_name("btn btn-primary");

        self.app.div.append_child(&h2_title)?;
        self.app.div.append_child(&logout_btn)?;

        let result_flash_message = data.into_serde::<FlashMessage>();

        if let Ok(flash_message) = result_flash_message {
            let flash_message_div = self.app.document.create_element("div")
                .expect("Creating alert not possible");
            flash_message_div.set_class_name("alert alert-info");
            flash_message_div.set_text_content(Some(&flash_message.message));
            self.app.div.append_child(&flash_message_div);
        }

        let button_et: EventTarget = logout_btn.into();
        let app_for_closure = self.app.clone();

        let handler = 
            Closure::wrap(Box::new(move || {
                let app_for_logout = app_for_closure.clone();

                let success_response = 
                    Closure::once(move |js_value: JsValue| {
                        let message = FlashMessage { message: "Logout Successful".to_string() };
                        components::routes::Routes::new(app_for_logout)
                            .go_to("/home".to_string(), &JsValue::from_serde(&message).unwrap());
                    });

                delete_request("auth")
                    .then(&success_response);
                success_response.forget();
            }) as Box<dyn FnMut()>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;
        handler.forget();

        Ok(())
    }

}
