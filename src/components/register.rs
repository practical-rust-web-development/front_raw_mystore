use std::sync::Arc;
use wasm_bindgen::{ JsValue, JsCast };
use wasm_bindgen::closure::Closure;
use web_sys::{ HtmlInputElement, HtmlButtonElement, HtmlFormElement, EventTarget };
use serde::{Deserialize, Serialize};
use crate::app::App;
use crate::components::component::{ Component, InputComponent };
use crate::fetch::post_request;

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

    fn load_components(&self) -> Result<(), JsValue> {

        let main_div = self.app.document.create_element("div")?;
        let h2_title = self.app.document.create_element("h2")?;
        h2_title.set_text_content(Some("Register an User"));

        let form = self.app.document.create_element("form")?;

        let email_div = self.app.document.create_element("div")?;
        email_div.set_class_name("from-group");
        let input_email_element = self.app.document.create_element("input")?;
        input_email_element.set_id("email");
        let input_email = JsCast::dyn_ref::<HtmlInputElement>(&input_email_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        input_email.set_placeholder("Email");
        email_div.append_child(input_email)?;

        let company_div = self.app.document.create_element("div")?;
        company_div.set_class_name("from-group");
        let input_company_element = self.app.document.create_element("input")?;
        input_company_element.set_id("company");
        let input_company = JsCast::dyn_ref::<HtmlInputElement>(&input_company_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        input_company.set_placeholder("Company");
        company_div.append_child(input_company)?;

        let password_div = self.app.document.create_element("div")?;
        password_div.set_class_name("from-group");
        let input_password_element = self.app.document.create_element("input")?;
        input_password_element.set_id("password");
        let input_password = JsCast::dyn_ref::<HtmlInputElement>(&input_password_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        input_password.set_placeholder("Password");
        password_div.append_child(input_password)?;

        let password_confirmation_div = self.app.document.create_element("div")?;
        password_confirmation_div.set_class_name("from-group");
        let input_password_confirmation_element = self.app.document.create_element("input")?;
        input_password_confirmation_element.set_id("password_confirmation");
        let input_password_confirmation = JsCast::dyn_ref::<HtmlInputElement>(&input_password_confirmation_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
        input_password_confirmation.set_placeholder("Password Confirmation");
        password_confirmation_div.append_child(input_password_confirmation)?;

        let button_element = self.app.document.create_element("button")?;
        let button = JsCast::dyn_ref::<HtmlButtonElement>(&button_element)
            .ok_or(JsValue::from_str("Error casting input"))?;
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
        let handler = 
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
                event.stop_propagation();
                post_request("register", &RegisterUser{
                    email: InputComponent(document.clone()).value_by_id("email"),
                    company: InputComponent(document.clone()).value_by_id("company"),
                    password: InputComponent(document.clone()).value_by_id("password"),
                    password_confirmation: InputComponent(document.clone()).value_by_id("password_confirmation")
                });
            }) as Box<dyn FnMut(_)>);

        button_et.add_event_listener_with_callback("click", handler.as_ref().unchecked_ref())?;

        handler.forget();

        self.app.div.append_child(&main_div)?;

        Ok(())
    }
}