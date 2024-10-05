use crate::api::get_course;
use crate::route::Route;
use crate::types::Course;
use anyhow::Error;
use wasm_bindgen::JsValue;
use web_sys::console;
use yew::prelude::*;
use yew_router::prelude::*;

struct State {
    course: Option<Course>,
    get_course_error: Option<Error>,
    get_course_loaded: bool,
}

pub struct CourseDetail {
    props: Props, // Now storing props
    state: State,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub id: usize,
}

pub enum Msg {
    GetCourse,
    GetCourseSuccess(Course),
    GetCourseError(Error),
    NavigateToHome,
}

impl Component for CourseDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        console::log_1(&format!("CourseDetail created with id: {}", ctx.props().id).into());
        let instance = Self {
            props: ctx.props().clone(),
            state: State {
                course: None,
                get_course_error: None,
                get_course_loaded: false,
            },
        };

        ctx.link().send_message(Msg::GetCourse);

        instance
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        console::log_1(&"Start of update function for course_detail".into());
        let mut res = true;
        match msg {
            Msg::GetCourse => {
                let id = self.props.id;
                console::log_1(&format!("Fetching course with id: {}", id).into());

                let handler =
                    ctx.link()
                        .callback(move |result: Result<Course, Error>| match result {
                            Ok(course) => Msg::GetCourseSuccess(course),
                            Err(err) => Msg::GetCourseError(err),
                        });

                get_course(id, handler);
            }
            Msg::GetCourseSuccess(course) => {
                self.state.course = Some(course);
                self.state.get_course_loaded = true;
            }
            Msg::GetCourseError(error) => {
                self.state.get_course_error = Some(error);
                self.state.get_course_loaded = true;
            }
            Msg::NavigateToHome => {
                ctx.link().navigator().unwrap().push(&Route::HomePage);
                res = false;
            }
        }
        res
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log_1(&"Start of view function for course_detail".into());
        if let Some(ref course) = self.state.course {
            console::log_1(&"Rendering course details".into());
            html! {
                <div class="course_detail_container">
                    <img class="course_detail_image" src={course.image.clone()}/>
                    <div class="course_card_name">{&course.name}</div>
                    <div class="course_card_price">{&course.teacher}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&course.description}</div>
                    <button class="course_atc_button" onclick={ctx.link().callback(|_| Msg::NavigateToHome)}>{"Return to MainPage"}</button>
                </div>
            }
        } else if !self.state.get_course_loaded {
            html! {
                <div class="loading_spinner_container">
                    <div class="loading_spinner"></div>
                    <div class="loading_spinner_text">{"Loading ..."}</div>
                </div>
            }
        } else {
            if let Some(ref err) = self.state.get_course_error {
                console::log_1(&JsValue::from(err.to_string()));
            }

            html! {
                <div>
                    <span>{"Error loading course! :("}</span>
                </div>
            }
        }
    }
}
