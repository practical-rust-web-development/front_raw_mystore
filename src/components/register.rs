use std::sync::Arc;
use serde_json::json;
use wasm_bindgen::{ JsValue, JsCast };
use wasm_bindgen::closure::Closure;
use web_sys::{ HtmlButtonElement, EventTarget, ErrorEvent };
use serde::{Deserialize, Serialize};
use crate::app::App;
use crate::components::component::{ Component, InputComponent, FlashMessage };
use crate::fetch::post_request;
use crate::components;

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String
}

impl RegisterUser {
    pub fn new() -> Self {
        RegisterUser {
            email: "".to_string(),
            company: "".to_string(),
            password: "".to_string(),
            password_confirmation: "".to_string()
        }
    }
}

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

    fn load_components(&self, data: &JsValue) -> Result<(), JsValue> {

        let main_div = self.app.document.create_element("div")?;
        main_div.set_class_name("container");
        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Register an User"));

        let form = self.app.document.create_element("form")?;

        let email_div = 
            InputComponent(self.app.document.clone())
                .create_input("email", "email", "text", "Email")?;

        let company_div = 
            InputComponent(self.app.document.clone())
                .create_input("company", "company", "text", "Company")?;

        let password_div = 
            InputComponent(self.app.document.clone())
                .create_input("password", "password", "password", "Password")?;

        let password_confirmation_div = 
            InputComponent(self.app.document.clone())
                .create_input("password_confirmation", "password_confirmation", "password", "Password Confirmation")?;

        let button_element = self.app.document.create_element("button")?;
        let button = JsCast::dyn_ref::<HtmlButtonElement>(&button_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        button.set_class_name("btn btn-primary");
        button.set_text_content(Some("Send"));
        button.set_type("Submit");

        form.append_child(&email_div)?;
        form.append_child(&company_div)?;
        form.append_child(&password_div)?;
        form.append_child(&password_confirmation_div)?;
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
                let register_user = RegisterUser{
                    email: InputComponent(document.clone()).value_by_id("email"),
                    company: InputComponent(document.clone()).value_by_id("company"),
                    password: InputComponent(document.clone()).value_by_id("password"),
                    password_confirmation: InputComponent(document.clone()).value_by_id("password_confirmation")
                };
                let serialized_register_user = json!(register_user).to_string();
                let app_success_closure = app_closure.clone();
                let success_response = 
                    Closure::once(move |js_value: JsValue| {
                        let message = FlashMessage { message: "User Created".to_string() };
                        components::routes::Routes::new(app_success_closure)
                            .go_to("/home".to_string(), &JsValue::from_serde(&message).unwrap());
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
                post_request("register", serialized_register_user)
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