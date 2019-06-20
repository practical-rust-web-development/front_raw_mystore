use std::sync::Arc;
use serde_json::json;
use wasm_bindgen::{ JsValue, JsCast };
use wasm_bindgen::closure::Closure;
use web_sys::{ HtmlButtonElement, EventTarget, ErrorEvent, Response };
use serde::{Deserialize, Serialize};
use crate::app::App;
use crate::components::component::{ Component, InputComponent, FlashMessage };
use crate::fetch::post_request;
use crate::components;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl LoginUser {
    pub fn new() -> Self {
        LoginUser {
            email: "".to_string(),
            password: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    email: String,
    company: String,
    created_at: String
}

#[derive(Clone)]
pub struct Login {
    url: String,
    app: Arc<App>
}

impl Login {
    pub fn new(url: String, app: Arc<App>) -> Self {
        Login { url, app }
    }
}

impl Component for Login {
    fn app(&self) -> Arc<App> { self.app.clone() }

    fn url(&self) -> String { self.url.clone() }

    fn load_components(&self, data: &JsValue) -> Result<(), JsValue> {

        let main_div = self.app.document.create_element("div")?;
        main_div.set_class_name("container");
        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Login"));

        let form = self.app.document.create_element("form")?;

        let email_div = 
            InputComponent(self.app.document.clone())
                .create_input("email", "email", "text", "Email")?;

        let password_div = 
            InputComponent(self.app.document.clone())
                .create_input("password", "password", "password", "Password")?;

        let button_element = self.app.document.create_element("button")?;
        let button = JsCast::dyn_ref::<HtmlButtonElement>(&button_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        button.set_class_name("btn btn-primary");
        button.set_text_content(Some("Send"));
        button.set_type("Submit");

        form.append_child(&email_div)?;
        form.append_child(&password_div)?;
        form.append_child(&button)?;

        main_div.append_child(&h2_title)?;
        main_div.append_child(&form)?;

        let button_et: EventTarget = button_element.into();

        let document = self.app.document.clone();
        let app_closure = self.app.clone();
        let form_closure = Arc::new(form);
        let handler = 
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                let login_user = LoginUser {
                    email: InputComponent(document.clone()).value_by_id("email"),
                    password: InputComponent(document.clone()).value_by_id("password")
                };
                let serialized_login_user = json!(login_user).to_string();
                let success_form_closure = form_closure.clone();
                let app_success_closure = app_closure.clone();
                let success_response = 
                    Closure::once(move |js_value: JsValue| {
                        let response_user = js_value.into_serde::<ResponseUser>();
                        if let Ok(user) = response_user {
                            let message = FlashMessage { message: "Login Successful".to_string() };
                            components::routes::Routes::new(app_success_closure)
                                .go_to("/dashboard".to_string(), &JsValue::from_serde(&message).unwrap());
                        } else {
                            let message = js_value.as_string().expect("Not a string");
                            let alert_error = app_success_closure.document.create_element("div")
                                .expect("Creating alert not possible");
                            alert_error.set_class_name("alert alert-danger");
                            alert_error.set_text_content(Some(&message));
                            success_form_closure.append_child(&alert_error);
                        }
                    });
                let error_form_closure = form_closure.clone();
                let app_error_closure = app_closure.clone();
                let error_response = 
                    Closure::once(move |js_value: JsValue| {
                        let response: &ErrorEvent = js_value.as_ref().unchecked_ref();
                        let text = response.message();
                        let alert_error = app_error_closure.document.create_element("div")
                            .expect("Creating alert not possible");
                        alert_error.set_class_name("alert alert-danger");
                        alert_error.set_text_content(Some(&text));
                        error_form_closure.append_child(&alert_error);
                    });
                post_request("auth", serialized_login_user)
                    .then(&success_response)
                    .catch(&error_response);
                error_response.forget();
                success_response.forget();
            }) as Box<dyn FnMut(_)>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();

        self.app.div.append_child(&main_div)?;

        Ok(())
    }
}
