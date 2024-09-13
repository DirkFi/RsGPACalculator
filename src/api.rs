// src/api.rs
use crate::types::Course;
use anyhow::Error;
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
// use yew::format::{Json, Nothing};
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};
// pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
// type FetchCallback<T> = Callback<FetchResponse<T>>;
//
// pub fn get_courses(callback: FetchCallback<Vec<Course>>) -> FetchTask {
//     let req = Request::get("/courses/courses.json").body(Nothing).unwrap();
//
//     FetchService::fetch(req, callback).unwrap()
// }

pub fn get_products(callback: Callback<Result<Vec<Course>, Error>>) {
    spawn_local(async move {
        let result = Request::get("/products/products.json")
            .send()
            .await
            .map_err(Error::from)
            .and_then(|resp| resp.json::<Vec<Course>>().await.map_err(Error::from));
        callback.emit(result);
    });
}
