// src/components/course_card.rs
use crate::route::Route;
use crate::types::Course;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct CourseCard;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub course: Course,
    pub grade: f32,
    pub grade_letter: String,
    pub check: bool,
    pub on_input_change: Callback<InputEvent>,
    pub on_select_change: Callback<Event>,
    pub on_toggle: Callback<()>,
}

impl Component for CourseCard {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.props().on_input_change.reform(|e: InputEvent| e);
        let onselect = ctx.props().on_select_change.reform(|e: Event| e);
        let ontoggle = ctx.props().on_toggle.reform(|_| ());

        html! {
            <div class="course_card_container">
                <Link<Route> to={Route::CourseDetail  { id: ctx.props().course.id }} classes="course_card_anchor">
                    <div class="course_card_name"> {&ctx.props().course.name}</div>
                    <div class="course_card_teacher"> {&ctx.props().course.teacher}</div>
                    <img class="course_card_image"  src={ctx.props().course.image.clone()}/>
                </Link<Route>>
                <br/>
                <div class="grade_input">
                    <input  type="number" value={ctx.props().grade.to_string()} step="any"  {oninput} />
                    <select onchange={onselect} value={ctx.props().grade_letter.clone()}>
                        <option value="A+">{"A+"}</option>
                        <option value="A">{"A"}</option>
                        <option value="A-">{"A-"}</option>
                        <option value="B+">{"B+"}</option>
                        <option value="B">{"B"}</option>
                        <option value="B-">{"B-"}</option>
                        <option value="C+">{"C+"}</option>
                        <option value="C">{"C"}</option>
                        <option value="F">{"F"}</option>
                    </select>
                    <span>
                        <input type="checkbox" onclick={ontoggle} checked={ctx.props().check}/>
                    </span>
                </div>
            </div>
        }
    }
}
