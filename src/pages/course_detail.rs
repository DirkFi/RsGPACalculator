use crate::api::get_course;
use crate::types::Course;
use anyhow::Error;
use yew::prelude::*;

struct State {
    course: Option<Course>,
    get_course_error: Option<Error>,
    get_course_loaded: bool,
}

pub struct CourseDetail {
    props: Props,
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
}

impl Component for CourseDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetCourse);

        Self {
            props: ctx.props().clone(),
            state: State {
                course: None,
                get_course_error: None,
                get_course_loaded: false,
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetCourse => {
                let handler =
                    ctx.link()
                        .callback(move |result: Result<Course, Error>| match result {
                            Ok(course) => Msg::GetCourseSuccess(course),
                            Err(err) => Msg::GetCourseError(err),
                        });

                get_course(self.props.id, handler);
                true
            }
            Msg::GetCourseSuccess(course) => {
                self.state.course = Some(course);
                self.state.get_course_loaded = true;
                true
            }
            Msg::GetCourseError(error) => {
                self.state.get_course_error = Some(error);
                self.state.get_course_loaded = true;
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(ref course) = self.state.course {
            html! {
                <div class="course_detail_container">
                    <img class="course_detail_image" src={course.image.clone()}/>
                    <div class="course_card_name">{&course.name}</div>
                    <div class="course_card_price">{"$"}{&course.teacher}</div>
                    <div style="margin: 10px 0; line-height: 24px;">{&course.description}</div>
                    <button class="course_atc_button">{"Add To Cart"}</button>
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
            html! {
                <div>
                    <span>{"Error loading course! :("}</span>
                </div>
            }
        }
    }
}
