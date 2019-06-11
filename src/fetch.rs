use futures::Future;
use js_sys::Promise;
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use serde::Serialize;

const BASE_URL: &str = "http://localhost:8088";

#[wasm_bindgen]
pub fn fetch_request(url: &str, method: &str, body: Option<String>) -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);
    if let Some(body_string) = body {
        let js_value = JsValue::from_str(&body_string);
        opts.body(Some(&js_value));
    }

    let request = Request::new_with_str_and_init(&format!("{}/{}", BASE_URL, url), &opts)?;

    request
        .headers()
        .set("Content-Type", "application/json")?;

    let window = web_sys::window().ok_or_else(|| JsValue::from_str("Could not get a window object"))?;
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into()?;
            resp.json()
        })
        .and_then(|json_value: Promise| {
            JsFuture::from(json_value)
        });

    Ok(future_to_promise(future))
}

pub fn post_request<T>(url: &str, body: &T) -> Result<Promise, JsValue> 
    where T: Serialize {

        let serialized_body = json!(body).to_string();
        fetch_request(url, "POST", Some(serialized_body))
}

pub fn get_request(url: &str) -> Result<Promise, JsValue>  {
    fetch_request(url, "GET", None)
}