// src/api.rs
use crate::types::Course;
use anyhow::{Error, Ok};
use gloo::net::http::Request;
// use reqwasm::http::Request;
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

pub fn get_courses(callback: Callback<Result<Vec<Course>, Error>>) {
    spawn_local(async move {
        let result = async {
            let response = Request::get("/courses/courses.json")
                .send()
                .await
                .map_err(Error::from)?;

            // Parse JSON as Vec<Course>
            let courses = response.json::<Vec<Course>>().await.map_err(Error::from)?;

            Ok(courses)
        }
        .await;

        // pass the res

        callback.emit(result);
    });
}

// pub fn get_product(id: i32, callback: FetchCallback<Product>) -> FetchTask {
//     let req = Request::get(format!("/products/{}.json", id))
//         .body(Nothing)
//         .unwrap();
//
//     FetchService::fetch(req, callback).unwrap()
// }

pub fn get_course(id: usize, callback: Callback<Result<Course, Error>>) {
    spawn_local(async move {
        let result = async {
            let response = Request::get(&format!("courses/{}.json", id))
                .send()
                .await
                .map_err(Error::from)?;
            let course = response.json::<Course>().await.map_err(Error::from)?;
            Ok(course)
        }
        .await;

        callback.emit(result);
    })
}
