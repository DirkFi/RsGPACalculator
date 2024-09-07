//src/pages/home.rs
use yew::html::Scope;
use yew::prelude::*;
#[derive(Clone, Properties, PartialEq)]
struct Course {
    id: i32,
    name: String,
    teacher: String,
    description: String,
    image: String,
}

struct Grade {
    course: Course,
    point: f32,
}

struct State {
    courses: Vec<Course>,
    grades: Vec<Grade>,
}

pub struct Home {
    state: State,
    link: Scope<Self>,
}

pub enum Msg {
    Chosen(i32),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let courses = vec![Course {
            id: 0,
            name: "Machine Learning".to_string(),
            teacher: "".to_string(),
            description: "".to_string(),
            image: "".to_string(),
        }];
        let grades = vec![];
        Self {
            state: State { courses, grades },
            link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Chosen(id) => {
                let course = self.state.courses.iter().find(|c| c.id == id).unwrap();

                let grade = self.state.grades.iter_mut().find(|g| g.course.id == id);
                if let Some(cg) = grade {
                    // update current course grade point to the new one
                    cg.point = 89.0;
                } else {
                    self.state.grades.push(Grade {
                        course: course.clone(),
                        point: 1.0,
                    })
                }
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let courses: Vec<Html> = self
            .state
            .courses
            .iter()
            .map(|course: &Course| {
                let course_id = course.id;
                html! {
                <div>
                <img src={course.image.clone()}/>

                <div> {course.name.clone()}</div>
                <div> {course.teacher.clone()}</div>
                <div> {course.description.clone()}</div>

                </div>
                }
            })
            .collect();
        html! { <span>{courses}</span> }
    }
}
