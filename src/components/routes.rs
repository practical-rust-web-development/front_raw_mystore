use std::collections::HashMap;
use std::sync::Arc;
use crate::components::component::Component;
use crate::components;
use crate::app::App;

pub struct Routes(HashMap<String, Box<Component>>);

impl Routes {
    pub fn new(app: Arc<App>) -> Routes {
        let mut routes = Routes(HashMap::new());
        routes.0.insert("/register".to_string(),
            Box::new(components::register::Register::new("register".to_string(), app.clone())));
        routes.0.insert("/home".to_string(),
            Box::new(components::home::Home::new("home".to_string(), app.clone())));
        routes.0.insert("/".to_string(),
            Box::new(components::home::Home::new("home".to_string(), app.clone())));
        routes
    }

    pub fn go_to(&self, url: String) {
        self.0.get(&url).expect("Component not created").render();
    }

    pub fn load_components(&self, url: String) {
        self.0.get(&url).expect("Component not created").load_components();
    }
}