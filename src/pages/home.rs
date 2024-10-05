//src/pages/home.rs
use crate::app_state::{AppStateAction, AppStateContext};
use crate::components::CourseCard;

use crate::api::get_courses;
use crate::types::Course;
use anyhow::Error;
use std::cmp::min;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
enum InnerRoute {
    #[at("/gpaview")]
    GradeView,
}

struct State {
    courses: Vec<Course>,
    grades: Vec<f32>,
    get_courses_error: Option<Error>,
    get_courses_loaded: bool,
    checks: Vec<bool>,
}

pub struct Home {
    state: State,
}

pub enum Msg {
    UpdateValue(usize, String),
    Chosen(usize),
    GetCourses,
    GetCoursesSuccess(Vec<Course>),
    GetCoursesError(Error),
}

pub fn point_to_pa(point: f32) -> f32 {
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

impl Home {
    fn calculate_gpa(&self) -> f32 {
        let mut numer: f32 = 0.0;
        let mut denomi: f32 = 0.0;
        for i in 0..self.state.courses.len() {
            if i < min(self.state.checks.len(), self.state.grades.len()) && self.state.checks[i] {
                numer += point_to_pa(self.state.grades[i]) * self.state.courses[i].unit as f32;
                denomi += self.state.courses[i].unit as f32;
            }
        }
        numer / denomi
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        console::log_1(&"Hello from Yew in creation!".into());
        let courses: Vec<Course> = vec![];
        ctx.link().send_message(Msg::GetCourses);

        let mut grades: Vec<f32> = vec![0.0; courses.len()];
        let mut checks: Vec<bool> = vec![false; courses.len()];

        console::log_1(&"hello after ctx sending links!".into());
        let (app_state, _) = ctx
            .link()
            .context::<AppStateContext>(Callback::noop())
            .expect("No AppStateContext found");

        if app_state.grades.len() > 0 {
            grades = (*app_state.grades).clone();
        }

        if app_state.checks.len() > 0 {
            checks = (*app_state.checks).clone();
        }
        Self {
            state: State {
                courses,
                grades,
                checks,
                get_courses_error: None,
                get_courses_loaded: false,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCourses => {
                self.state.get_courses_loaded = false;
                let handler = ctx
                    .link()
                    .callback(move |result: Result<Vec<Course>, Error>| match result {
                        Ok(courses) => Msg::GetCoursesSuccess(courses),
                        Err(err) => Msg::GetCoursesError(err),
                    });
                get_courses(handler);
                true
            }

            Msg::GetCoursesSuccess(courses) => {
                console::log_1(&"hello from GetCoursesSuccess!".into());
                self.state.courses = courses;
                if self.state.grades.is_empty(){
                    self.state.grades = vec![0.0; self.state.courses.len()];
                }
                if self.state.checks.is_empty(){
                    self.state.checks = vec![false; self.state.courses.len()];
                }
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

                if self.state.get_courses_loaded {
                    console::log_1(&"Change inside Home is qidong after if!".into());
                    let (app_state, _context_handle) = ctx
                        .link()
                        .context::<AppStateContext>(Callback::noop())
                        .expect("No AppStateContext found");

                    let courses = Rc::new(self.state.courses.clone());
                    let grades = Rc::new(self.state.grades.clone());
                    let checks = Rc::new(self.state.checks.clone());

                    app_state.dispatch(AppStateAction::UpdateAll {
                        courses,
                        grades,
                        checks,
                    });
                }
                true
            }

            Msg::Chosen(id) => {
                if let Some(check_box) = self.state.checks.get_mut(id) {
                    *check_box = !*check_box;
                } else {
                    self.state.checks.push(true);
                }
                if self.state.get_courses_loaded {
                    console::log_1(&"Change inside Home is qidong after if!".into());
                    let (app_state, _context_handle) = ctx
                        .link()
                        .context::<AppStateContext>(Callback::noop())
                        .expect("No AppStateContext found");

                    let courses = Rc::new(self.state.courses.clone());
                    let grades = Rc::new(self.state.grades.clone());
                    let checks = Rc::new(self.state.checks.clone());

                    app_state.dispatch(AppStateAction::UpdateAll {
                        courses,
                        grades,
                        checks,
                    });
                }

                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let courses_html: Vec<Html> = self
            .state
            .courses
            .iter()
            .enumerate()
            .map(|(index, course): (usize, &Course)| {
                let oninput = ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();

                    Msg::UpdateValue(index, input.value())
                });

                let ontoggle = ctx.link().callback(move |_| Msg::Chosen(index));
                html! {
                <CourseCard course={course.clone()} grade={self.state.grades[index]} 
                    check={self.state.checks[index]} on_input_change={oninput} on_toggle={ontoggle}/>
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
                <span>{"Error loading courses! :("}</span>
              </div>
            }
        } else {
            html! {
                <div>
                    <div class="navbar">
                        <div class="navbar_title"> {"GPA Calculator written in Rust"}</div>
                        <div class="navbar_value"> {self.calculate_gpa()}</div>

                    </div>
                    <div>
                        <span class="course_card_list">{courses_html}</span>
                        <Link<InnerRoute> to={InnerRoute::GradeView }>
                            <button>{"Generate"}</button>
                        </Link<InnerRoute>>
                        <p>{self.calculate_gpa()} </p>
                    </div>

                </div>

            }
        }
        // TODO:
        // idea:
        // 2. Important! add one add function to manually add any course that is not in json file
        // 3. seperate courses section based on different semesters
        // 4. teacher intro goes to ratemyprof?
    }
}
