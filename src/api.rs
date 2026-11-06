// src/api.rs
use crate::types::Course;
use anyhow::{anyhow, Error, Ok};
use gloo::net::http::Request;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

const COURSES_URL: &str = "courses/courses.json";

// Shared fetch + parse logic so both entry points use the same code path.
async fn fetch_courses() -> Result<Vec<Course>, Error> {
    let response = Request::get(COURSES_URL).send().await.map_err(Error::from)?;
    let courses = response.json::<Vec<Course>>().await.map_err(Error::from)?;
    Ok(courses)
}

pub fn get_courses(callback: Callback<Result<Vec<Course>, Error>>) {
    spawn_local(async move {
        callback.emit(fetch_courses().await);
    });
}

pub fn get_course(id: usize, callback: Callback<Result<Course, Error>>) {
    spawn_local(async move {
        let result = async {
            let courses = fetch_courses().await?;
            // Avoid panicking on out-of-range ids (a panic would kill the whole WASM app).
            courses
                .get(id)
                .cloned()
                .ok_or_else(|| anyhow!("course {} not found", id))
        }
        .await;

        callback.emit(result);
    })
}
