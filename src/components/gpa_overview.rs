// src/components/gpa_overview.rs
use crate::app_state::AppStateContext;
use crate::pages::point_to_pa;
use crate::route::Route;
use crate::types::Course;
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(GPAOverview)]
pub fn gpa_overview() -> Html {
    let app_state = use_context::<AppStateContext>().expect("No AppStateContext found");

    // Combine fetched and user-added courses
    let all_courses: Vec<&Course> = app_state
        .courses
        .iter()
        .chain(app_state.user_courses.iter())
        .collect();

    let all_grades: Vec<f32> = app_state
        .grades
        .iter()
        .copied()
        .chain(app_state.user_grades.iter().copied())
        .collect();

    let all_checks: Vec<bool> = app_state
        .checks
        .iter()
        .copied()
        .chain(app_state.user_checks.iter().copied())
        .collect();

    // Calculate GPA
    let mut numer: f32 = 0.0;
    let mut denomi: f32 = 0.0;
    for i in 0..all_courses.len() {
        if all_checks[i] {
            numer += point_to_pa(all_grades[i]) * all_courses[i].unit as f32;
            denomi += all_courses[i].unit as f32;
        }
    }
    let gpa = if denomi != 0.0 { numer / denomi } else { 0.0 };

    // Render the courses
    let courses_view: Html = all_courses
        .iter()
        .enumerate()
        .filter(|(i, _)| all_checks[*i])
        .map(|(i, course)| {
            let grade = all_grades[i];
            html! {
                <tr>
                    <td style="padding: 10px; text-align: center;">{&course.name}</td>
                    <td style="padding: 10px; text-align: center;">{grade}</td>
                    <td style="padding: 10px; text-align: center;">{&course.unit}</td>
                </tr>
            }
        })
        .collect();

    // Render the component
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
                {format!("Overall GPA is：{:.2}", gpa)}
            </div>

            <Link<Route>  to={Route::HomePage } >
                <div class="gpa-button-container">
                    <button class="course_atc_button">{"Return to MainPage"}</button>
                </div>
            </Link<Route>>
        </div>
    }
}
