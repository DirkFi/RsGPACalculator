// src/pages/home.rs

use crate::api::get_courses;
use crate::app_state::{AppState, AppStateAction, AppStateContext, AppStateValue};
use crate::components::CourseCard;
use crate::types::Course;
use anyhow::Error;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::window;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
enum InnerRoute {
    #[at("/gpaview")]
    GradeView,
}

struct State {
    get_courses_error: Option<Error>,
    get_courses_loaded: bool,
}

pub struct Home {
    state: State,
    theme: String,
}

pub enum Msg {
    UpdateValue(usize, String),
    ToggleCourseCheck(usize),
    GetCourses,
    GetCoursesSuccess(Vec<Course>),
    GetCoursesError(Error),
    AddNewCourseCard,
    UpdateUserCourseName(usize, String),
    UpdateUserCourseUnit(usize, String),
    UpdateUserCourseGrade(usize, String),
    ToggleUserCourseCheck(usize),
    ToggleTheme,
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
    fn calculate_gpa(&self, ctx: &Context<Self>) -> f32 {
        let mut numer: f32 = 0.0;
        let mut denomi: f32 = 0.0;

        // Include fetched courses

        let (app_state, _context_handle) = ctx
            .link()
            .context::<AppStateContext>(Callback::noop())
            .expect("No AppStateContext found");

        for i in 0..app_state.checks.len() {
            if app_state.checks[i] {
                numer += point_to_pa(app_state.grades[i]) * app_state.courses[i].unit as f32;
                denomi += app_state.courses[i].unit as f32;
            }
        }

        // Include user-added courses
        for i in 0..app_state.user_checks.len() {
            if app_state.user_checks[i] {
                numer +=
                    point_to_pa(app_state.user_grades[i]) * app_state.user_courses[i].unit as f32;
                denomi += app_state.user_courses[i].unit as f32;
            }
        }

        if denomi != 0.0 {
            numer / denomi
        } else {
            0.0
        }
    }

    fn view_user_course_card(
        &self,
        ctx: &Context<Self>,
        app_state: &UseReducerHandle<AppState>,
        index: usize,
        course: &Course,
    ) -> Html {
        // let (app_state, _context_handle) = ctx
        //     .link()
        //     .context::<AppStateContext>(Callback::noop())
        //     .expect("No AppStateContext found");
        let user_grades = (*app_state.user_grades).clone();
        let user_checks = (*app_state.user_checks).clone();
        let on_name_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateUserCourseName(index, input.value())
        });

        let on_unit_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateUserCourseUnit(index, input.value())
        });

        let on_grade_input = ctx.link().callback(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateUserCourseGrade(index, input.value())
        });

        let on_toggle = ctx
            .link()
            .callback(move |_| Msg::ToggleUserCourseCheck(index));

        html! {
            <div class="course_card_container">
                <div class="form-group">
                    <label for="cname">{ "Course Name: " }</label>
                    <input
                        type="text"
                        id="cname"
                        placeholder="Course Name"
                        value={course.name.clone()}
                        oninput={on_name_input}
                    />
                    <br/>
                </div>


                <div class="form-group">
                    <label for="unit">{ "Unit: " }</label>
                    <input
                        type="text"
                        id="unit"
                        placeholder="Unit"
                        value={course.unit.to_string()}
                        oninput={on_unit_input}
                    />
                    <br/>
                </div>

                <div class="form-group">
                    <label for="grade">{ "Grade: " }</label>
                    <input
                        type="text"
                        id="grade"
                        placeholder="Grade"
                        value={user_grades[index].to_string()}
                        oninput={on_grade_input}
                    />
                </div>
                <input
                    type="checkbox"
                    checked={user_checks[index]}
                    onclick={on_toggle}
                />
            </div>
        }
    }

    fn update_app_user_state(
        &self,
        ctx: &Context<Self>,
        user_courses: Vec<Course>,
        user_grades: Vec<f32>,
        user_checks: Vec<bool>,
    ) {
        if self.state.get_courses_loaded {
            console::log_1(&"Updating AppState with non-user courses.".into());
            let (app_state, _context_handle) = ctx
                .link()
                .context::<AppStateContext>(Callback::noop())
                .expect("No AppStateContext found");

            app_state.dispatch(AppStateAction::UpdateAllUser {
                user_courses: Rc::new(user_courses.clone()),
                user_grades: Rc::new(user_grades.clone()),
                user_checks: Rc::new(user_checks.clone()),
            });
        }
    }

    fn update_app_single_state(&self, ctx: &Context<Self>, values: AppStateValue) {
        let (app_state, _context_handle) = ctx
            .link()
            .context::<AppStateContext>(Callback::noop())
            .expect("No AppStateContext found");

        app_state.dispatch(AppStateAction::UpdateSingle { values });
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        console::log_1(&"Hello from Yew in creation!".into());
        ctx.link().send_message(Msg::GetCourses);
        let window = web_sys::window().expect("window should exist");
        // Get the initial theme, default to "light"
        let initial_theme = {
            if let Some(storage) = window.local_storage().ok().flatten() {
                match storage.get_item("theme") {
                    Ok(Some(theme)) => theme,
                    _ => "light".to_string(),
                }
            } else {
                "light".to_string()
            }
        };

        // Set the theme on the body element
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.set_class_name(&initial_theme);
        Self {
            state: State {
                get_courses_error: None,
                get_courses_loaded: false,
            },
            theme: initial_theme,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (app_state, _context_handle) = ctx
            .link()
            .context::<AppStateContext>(Callback::noop())
            .expect("No AppStateContext found");
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
            }

            Msg::GetCoursesSuccess(courses) => {
                console::log_1(&"hello from GetCoursesSuccess!".into());
                self.state.get_courses_loaded = true;

                let grades = if !app_state.grades.is_empty() {
                    (*app_state.grades).clone()
                } else {
                    vec![0.0; courses.len()]
                };

                let checks = if !app_state.checks.is_empty() {
                    (*app_state.checks).clone()
                } else {
                    vec![false; courses.len()]
                };
                app_state.dispatch(AppStateAction::UpdateAllNonUser {
                    courses: Rc::new(courses),
                    grades: Rc::new(grades),
                    checks: Rc::new(checks),
                });
            }

            Msg::GetCoursesError(error) => {
                self.state.get_courses_error = Some(error);
                self.state.get_courses_loaded = true;
            }
            // Need to change this from self to mutual context
            Msg::UpdateValue(index, value) => {
                let possible_num = value.parse::<f32>();
                let mut grades = (*app_state.grades).clone();
                match possible_num {
                    Ok(num) => {
                        if let Some(cg) = grades.get_mut(index) {
                            // update current course grade point to the new one
                            *cg = num;
                        } else {
                            grades.push(num);
                        }
                    }
                    Err(_) => {
                        console::log_1(&"You should only type numbers here!".into());
                    }
                }
                console::log_1(&"Current point is: ".into());
                console::log_1(&JsValue::from(grades[index]));

                if self.state.get_courses_loaded {
                    self.update_app_single_state(ctx, AppStateValue::Grades(grades));
                }
            }

            Msg::ToggleCourseCheck(id) => {
                let mut checks = (*app_state.checks).clone();
                if let Some(check_box) = checks.get_mut(id) {
                    *check_box = !*check_box;
                } else {
                    checks.push(true);
                }
                if self.state.get_courses_loaded {
                    console::log_1(&"Change inside Home is qidong after if!".into());
                    self.update_app_single_state(ctx, AppStateValue::Checks(checks));
                }
            }

            Msg::AddNewCourseCard => {
                // Add a new empty course to user_courses
                let courses_len = app_state.courses.len();
                let mut user_courses = (*app_state.user_courses).clone();
                let mut user_grades = (*app_state.user_grades).clone();
                let mut user_checks = (*app_state.user_checks).clone();
                user_courses.push(Course {
                    id: courses_len + user_courses.len(),
                    teacher: "".to_string(),
                    description: "".to_string(),
                    image: "".to_string(),
                    name: String::new(),
                    unit: 0,
                    // Other fields if necessary
                });
                user_grades.push(0.0);
                user_checks.push(false);
                // Update the AppState context
                self.update_app_user_state(ctx, user_courses, user_grades, user_checks);
            }
            Msg::UpdateUserCourseName(index, name) => {
                let mut user_courses = (*app_state.user_courses).clone();
                if let Some(course) = user_courses.get_mut(index) {
                    course.name = name;
                }
                self.update_app_single_state(ctx, AppStateValue::UserCourses(user_courses));
            }
            Msg::UpdateUserCourseUnit(index, unit_str) => {
                let mut user_courses = (*app_state.user_courses).clone();
                if let Ok(unit) = unit_str.parse::<i32>() {
                    if let Some(course) = user_courses.get_mut(index) {
                        course.unit = unit;
                    }
                }
                self.update_app_single_state(ctx, AppStateValue::UserCourses(user_courses));
            }
            Msg::UpdateUserCourseGrade(index, grade_str) => {
                let mut user_grades = (*app_state.user_grades).clone();
                if let Ok(grade) = grade_str.parse::<f32>() {
                    if let Some(grade_slot) = user_grades.get_mut(index) {
                        *grade_slot = grade;
                    }
                }

                self.update_app_single_state(ctx, AppStateValue::UserGrades(user_grades));
            }
            Msg::ToggleUserCourseCheck(index) => {
                let mut user_checks = (*app_state.user_checks).clone();
                if let Some(check) = user_checks.get_mut(index) {
                    *check = !*check;
                }

                self.update_app_single_state(ctx, AppStateValue::UserChecks(user_checks));
            }
            Msg::ToggleTheme => {
                // Toggle between light and dark themes
                let new_theme = if self.theme == "light" {
                    "dark".to_string()
                } else {
                    "light".to_string()
                };

                // Update theme state
                self.theme = new_theme.clone();

                // Save to localStorage
                let window = window().unwrap();
                let storage = window.local_storage().unwrap().unwrap();
                storage.set_item("theme", &new_theme).unwrap();

                // Update body class
                let document = window.document().unwrap();
                let icon = document
                    .get_element_by_id("themeIcon")
                    .expect("theme icon should exist");
                let icon = icon.dyn_into::<web_sys::HtmlElement>().unwrap();
                if new_theme == "dark" {
                    icon.set_class_name("fas fa-sun");
                } else {
                    icon.set_class_name("fas fa-moon");
                }
                let body = document.body().unwrap();
                body.set_class_name(&new_theme);
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (app_state, _context_handle) = ctx
            .link()
            .context::<AppStateContext>(Callback::noop())
            .expect("No AppStateContext found");
        let courses_html: Vec<Html> = app_state
            .courses
            .iter()
            .enumerate()
            .map(|(index, course): (usize, &Course)| {
                let oninput = ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();

                    Msg::UpdateValue(index, input.value())
                });

                let ontoggle = ctx.link().callback(move |_| Msg::ToggleCourseCheck(index));
                html! {
                <CourseCard course={course.clone()} grade={app_state.grades[index]}
                    check={app_state.checks[index]} on_input_change={oninput} on_toggle={ontoggle}/>
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
            let toggle_theme = ctx.link().callback(|_| Msg::ToggleTheme);
            html! {
                <div>
                    <div class="navbar">
                        <div class="navbar_title"> {"GPA Calculator written in Rust"}</div>
                        <div class="right-section">
                            <button id="themeToggleButton" onclick={toggle_theme}>
                                //{ if self.theme == "light" { "Switch to Dark Mode" } else { "Switch to Light Mode" } }
                                <i id="themeIcon" class="fas fa-moon"></i>
                            </button>
                            <div class="navbar_value"> {"GPA: "}{self.calculate_gpa(ctx)}</div>
                        </div>
                    </div>
                    <div>
                        <span class="course_card_list">{courses_html}</span>
                        // Button to add a new CourseCard
                        <div class="home-button">
                            <button class="button-28" onclick={ctx.link().callback(|_| Msg::AddNewCourseCard)}>
                                { "Add New Course" }
                            </button>
                        </div>

                        // Render the list of user-added CourseCards
                        <div class="course_card_list">
                            { for app_state.user_courses.iter().enumerate().map(|(index, course)| {
                                self.view_user_course_card(ctx, &app_state, index, course)
                            })}
                        </div>
                        <Link<InnerRoute> to={InnerRoute::GradeView }>
                            <div class="generate-button">
                                <button class="button-28">{"Generate"}</button>
                            </div>
                         </Link<InnerRoute>>
                     </div>

                </div>

            }
        }
        // TODO:
        // idea:
        // 1. Change the context update to be cleaner
        // 2. seperate courses section based on different semesters
        // 3. teacher intro goes to ratemyprof?
    }
}
