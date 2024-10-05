// src/components/gpa_overview.rs
use crate::app_state::AppStateContext;
use crate::pages::point_to_pa;
use std::cmp::min;
use yew::prelude::*;

use wasm_bindgen::JsValue;
use web_sys::console;

#[function_component(GPAOverview)]
pub fn gpa_overview() -> Html {
    let app_state = use_context::<AppStateContext>().expect("No AppStateContext found");

    console::log_1(&"Current courses in gpa_overview is: ".into());
    console::log_1(&JsValue::from_str(&format!(
        "Courses: {:?}, Grades: {:?}, Checks: {:?}",
        app_state.courses, app_state.grades, app_state.checks
    )));
    let calculate_gpa = || -> f32 {
        let mut numer: f32 = 0.0;
        let mut denomi: f32 = 0.0;
        for i in 0..app_state.courses.len() {
            if i < min(app_state.checks.len(), app_state.grades.len()) && app_state.checks[i] {
                numer += point_to_pa(app_state.grades[i]) * app_state.courses[i].unit as f32;
                denomi += app_state.courses[i].unit as f32;
            }
        }
        numer / denomi
    };

    let courses_view: Html = app_state
        .courses
        .iter()
        .enumerate()
        .filter(|(i, _)| app_state.checks.get(*i).copied().unwrap_or(false))
        .map(|(i, course)| {
            let grade = app_state.grades.get(i).copied().unwrap_or(0.0);
            html! {
                <tr>
                    <td style="padding: 10px; text-align: center;">{&course.name}</td>
                    <td style="padding: 10px; text-align: center;">{grade}</td>
                    <td style="padding: 10px; text-align: center;">{&course.unit}</td>
                </tr>
            }
        })
        .collect();

    html! {
        <div>
            <h2 style="text-align: center;">{"GPA Overview"}</h2>
            <table style="
                width: 60%; 
                border-collapse: collapse; 
                margin: 20px auto; 
                border: 1px solid #333;">
                <tr style="background-color: #f2f2f2;">
                    <th style="padding: 10px; text-align: center;">{"Course Name"}</th>
                    <th style="padding: 10px; text-align: center;">{"Course Grade"}</th>
                    <th style="padding: 10px; text-align: center;">{"Unit"}</th>
                </tr>
                {courses_view}
            </table>
            <div style="font-size: 18px; font-weight: bold; text-align: center; margin-top: 20px;">
                {format!("Overall GPA is：{}", calculate_gpa())}
            </div>
        </div>
    }
}
