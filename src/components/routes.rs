use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use crate::components::component::Component;
use crate::components;
use crate::app::App;

pub struct Routes(HashMap<String, Box<Component>>);

impl Routes {
    pub fn new(app: Arc<App>) -> Routes {
        let mut routes = Routes(HashMap::new());
        routes.0.insert("/dashboard".to_string(),
            Box::new(components::dashboard::Dashboard::new("dashboard".to_string(), app.clone())));
        routes.0.insert("/login".to_string(),
            Box::new(components::login::Login::new("login".to_string(), app.clone())));
        routes.0.insert("/register".to_string(),
            Box::new(components::register::Register::new("register".to_string(), app.clone())));
        routes.0.insert("/home".to_string(),
            Box::new(components::home::Home::new("home".to_string(), app.clone())));
        routes.0.insert("/".to_string(),
            Box::new(components::home::Home::new("home".to_string(), app.clone())));
        routes
    }

    pub fn go_to(&self, url: String, state: &JsValue) {
        self.0.get(&url).expect("Component not created").render(state);
    }

    pub fn load_components(&self, url: String, state: &JsValue ) {
        self.0.get(&url).expect("Component not created").load_components(state);
    }
}