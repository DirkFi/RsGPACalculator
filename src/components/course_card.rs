//src/components/course_card.rs
use crate::route::Route;
use crate::types::Course;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct CourseCard {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub course: Course,
    pub on_input_change: Callback<InputEvent>,
    pub on_toggle: Callback<()>,
}

impl Component for CourseCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let oninput = self.props.on_input_change.reform(|e: InputEvent| e);
        let ontoggle = self.props.on_toggle.reform(|_| ());

        html! {
        <div class="course_card_container">
            <Link<Route> to={Route::CourseDetail  { id: self.props.course.id }} classes="course_card_anchor">
                <img class="course_card_image" src={self.props.course.image.clone()}/>

                <div class="course_card_name"> {self.props.course.name.clone()}</div>
                <div class="course_card_teacher"> {self.props.course.teacher.clone()}</div>
                // <div class="course_card_desp"> {self.props.course.description.clone()}</div>
            </Link<Route>>
            <div>
                <input type="number" step="any"  {oninput} />
                <input type="checkbox"  onclick={ontoggle} />
            </div>
        </div>
        }
    }
}
