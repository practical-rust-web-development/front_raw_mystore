use futures::{future, Future};
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
pub struct Branch {
    pub name: String,
    pub commit: Commit,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub commit: CommitDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitDetails {
    pub author: Signature,
    pub committer: Signature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Signature {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub company: String,
    pub password: String,
    pub password_confirmation: String
}

#[wasm_bindgen]
pub fn run() -> Promise {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let signature = json!(RegisterUser { 
        company: "peter".to_string(),
        email: "peter@email.com".to_string(),
        password: "12345678".to_string(),
        password_confirmation: "12345678".to_string() }).to_string();
    let serialized_signature = JsValue::from_str(&signature);
    opts.body(Some(&serialized_signature));

    let request = Request::new_with_str_and_init(
        "http://localhost:8088/register",
        &opts,
    )
    .unwrap();

    request
        .headers()
        .set("Content-Type", "application/json")
        .unwrap();

    let window = web_sys::window().unwrap();
    let request_promise = window.fetch_with_request(&request);

    let future = JsFuture::from(request_promise)
        .and_then(|resp_value| {
            assert!(resp_value.is_instance_of::<Response>());
            let resp: Response = resp_value.dyn_into().unwrap();
            resp.json()
        })
        .and_then(|json_value: Promise| {
            JsFuture::from(json_value)
        })
        .and_then(|json| {
            let branch_info: Branch = json.into_serde().unwrap();

            future::ok(JsValue::from_serde(&branch_info).unwrap())
        });

    future_to_promise(future)
}