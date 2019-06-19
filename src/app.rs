use wasm_bindgen::JsValue;
use web_sys::{ Window, Document, Element, window };
use std::sync::Arc;
use crate::router::Router;

pub struct App {
    pub div: Arc<Element>,
    pub window: Arc<Window>,
    pub document: Arc<Document>,
    pub router: Arc<Router>
}

impl App {
    pub fn new() -> Self {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");

        let app = document.get_element_by_id("app").expect("A div with id app is required");

        App {
            window: Arc::new(window),
            div: Arc::new(app),
            document: Arc::new(document),
            router: Arc::new(Router::new()) 
        }
    }

    pub fn go_to(&self, url: &str, state: &JsValue) -> Result<(), JsValue> {
        self.router.go_to(url, state)
    }
}