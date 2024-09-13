//src/pages/home.rs
use crate::api;
use crate::types::Course;
use anyhow::Error;
use std::cmp::min;
use wasm_bindgen::JsValue;
use web_sys::{console, HtmlInputElement};
use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

// #[derive(Clone, Properties, PartialEq)]
// struct Course {
//     id: usize,
//     name: String,
//     teacher: String,
//     description: String,
//     image: String,
//     unit: i32,
// }
//
struct State {
    courses: Vec<Course>,
    grades: Vec<f32>,
    get_courses_error: Option<Error>,
    get_courses_loaded: bool,
    checks: Vec<bool>,
}

pub struct Home {
    state: State,
    task: Option<FetchTask>,
}

pub enum Msg {
    UpdateValue(usize, String),
    Chosen(usize),
    GetCourses,
    GetCoursesSuccess(Vec<Course>),
    GetCoursesError(Error),
}

impl Home {
    fn calculate_gpa(&self) -> f32 {
        let mut numer: f32 = 0.0;
        let mut denomi: f32 = 0.0;
        for i in 0..self.state.courses.len() {
            if i < min(self.checks.len(), self.state.grades.len()) && self.checks[i] {
                numer += self.point_to_pa(self.state.grades[i]) * self.state.courses[i].unit as f32;
                denomi += self.state.courses[i].unit as f32;
            }
        }
        numer / denomi
    }

    fn point_to_pa(&self, point: f32) -> f32 {
        match point {
            95.0..=100.0 => 4.33,
            90.0..=95.0 => 4.0,
            85.0..90.0 => 3.67,
            80.0..85.0 => 3.33,
            75.0..80.0 => 3.0,
            70.0..75.0 => 2.67,
            65.0..70.0 => 2.33,
            60.0..65.0 => 2.0,
            55.0..60.0 => 1.67,
            50.0..55.0 => 1.0,
            0.0..50.0 => 0.0,
            _ => -1.0,
        }
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console::log_1(&"Hello from Yew in creation!".into());
        let courses: Vec<Course> = vec![];
        let grades: Vec<f32> = vec![0.0; courses.len()];
        let checks: Vec<bool> = vec![false; courses.len()];
        self.link.send_message(Msg::GetCourses);
        Self {
            state: State {
                courses,
                grades,
                checks,
                get_courses_error: None,
                get_courses_loaded: false,
            },
            task: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCourses => {
                self.state.get_courses_loaded = false;
                let handler =
                    self.link
                        .callback(move |response: api::FetchResponse<Vec<Course>>| {
                            let (_, Json(data)) = response.into_parts();
                            match data {
                                Ok(products) => Msg::GetCoursessSuccess(products),
                                Err(err) => Msg::GetCoursesError(err),
                            }
                        });
                self.task = Some(api::get_courses(handler));
                true
            }

            Msg::GetCoursesSuccess(courses) => {
                self.state.courses = courses;
                self.state.get_courses_loaded = true;
                true
            }
            Msg::GetCoursesError(error) => {
                self.state.get_courses_error = Some(error);
                self.state.get_courses_loaded = true;
                true
            }
            Msg::UpdateValue(index, value) => {
                let possible_num = value.parse::<f32>();
                match possible_num {
                    Ok(num) => {
                        if let Some(cg) = self.state.grades.get_mut(index) {
                            // update current course grade point to the new one
                            *cg = num;
                        } else {
                            self.state.grades.push(num);
                        }
                    }
                    Err(_) => {
                        console::log_1(&"You should only type numbers here!".into());
                    }
                }
                console::log_1(&"Current point is: ".into());
                console::log_1(&JsValue::from(self.state.grades[index]));
                true
            }

            Msg::Chosen(id) => {
                if let Some(check_box) = self.state.checks.get_mut(id) {
                    *check_box = !*check_box;
                } else {
                    self.checks.push(true);
                }
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let courses: Vec<Html> = self
            .state
            .courses
            .iter()
            .map(|course: &Course| {
                let index = course.id;
                let oninput = ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();

                    Msg::UpdateValue(index, input.value())
                });

                let ontoggle = ctx.link().callback(move |_| Msg::Chosen(index));
                html! {
                <div>
                    <img src={course.image.clone()}/>

                    <div> {course.name.clone()}</div>
                    <div> {course.teacher.clone()}</div>
                    <div> {course.description.clone()}</div>
                    <div>
                        <input type="number" step="any"  {oninput} />
                        <input type="checkbox"  onclick={ontoggle} />
                    </div>
                </div>
                }
            })
            .collect();
        if !self.state.get_courses_loaded {
            html! {
              <div>{"Loading ..."}</div>
            }
        } else if let Some(_) = self.state.get_courses_error {
            html! {
              <div>
                <span>{"Error loading products! :("}</span>
              </div>
            }
        } else {
            html! {
                <div>
                    <span>{courses}</span>
                    <button>{"Generate"}</button>
                    <p>{self.calculate_gpa()} </p>
                </div>
            }
        }
    }
}
