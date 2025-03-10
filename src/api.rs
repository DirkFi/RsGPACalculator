// src/api.rs
use crate::types::Course;
use anyhow::{Error, Ok};
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

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

pub fn get_course(id: usize, callback: Callback<Result<Course, Error>>) {
    spawn_local(async move {
        let result = async {
            let response = Request::get("/courses/courses.json")
                .send()
                .await
                .map_err(Error::from)?;

            // Parse JSON as Vec<Course>
            let courses = response.json::<Vec<Course>>().await.map_err(Error::from)?;
            let course = courses[id].clone();
            Ok(course)
        }
        .await;

        callback.emit(result);
    })
}
