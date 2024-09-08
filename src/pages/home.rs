//src/pages/home.rs
use web_sys::{console, HtmlInputElement};
use yew::html::Scope;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Course {
    id: usize,
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
    checks: Vec<bool>,
    link: Scope<Self>,
}

pub enum Msg {
    UpdateValue(usize, String),
    Chosen(usize),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        console::log_1(&"Hello from Yew in creation!".into());
        let link = ctx.link().clone();
        let courses = vec![Course {
            id: 0,
            name: "Machine Learning".to_string(),
            teacher: "".to_string(),
            description: "".to_string(),
            image: "".to_string(),
        }];
        let grades: Vec<Grade> = vec![];
        let checks: Vec<bool> = vec![];
        Self {
            state: State { courses, grades },
            checks,
            link,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateValue(index, value) => {
                console::log_1(&"Hello from update 1st!".into());
                if let Some(check_box) = self.checks.get(index) {
                    if *check_box {
                        let course = self.state.courses.iter().find(|c| c.id == index).unwrap();
                        let grade = self.state.grades.iter_mut().find(|g| g.course.id == index);
                        if let Some(cg) = grade {
                            // update current course grade point to the new one
                            cg.point = value.parse::<f32>().unwrap();
                        } else {
                            self.state.grades.push(Grade {
                                course: course.clone(),
                                point: value.parse::<f32>().unwrap(),
                            });
                        }
                        console::log_1(&"Hello from Yew!".into());
                    }
                }
                true
            }

            Msg::Chosen(id) => {
                console::log_1(&"Hello from update 2nd!".into());
                if let Some(check_box) = self.checks.get_mut(id) {
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
                let idx = course.id;
                let mut point = String::from("0.0");
                let mut checked = false;
                if idx < self.state.grades.len() {
                    point = self.state.grades[idx].point.to_string();
                }
                if idx < self.checks.len() {
                    checked = self.checks[idx];
                }
                let oninput = ctx.link().callback(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    Msg::UpdateValue(idx, input.value())
                });

                let ontoggle = ctx.link().callback(move |_| Msg::Chosen(idx));
                html! {
                     <div>
                     <img src={course.image.clone()}/>

                     <div> {course.name.clone()}</div>
                     <div> {course.teacher.clone()}</div>
                     <div> {course.description.clone()}</div>
                <div>
                    <input type="number" step="any"  {oninput} />
                    <input type="checkbox"  {ontoggle} />
                </div>
                </div>
                     }
            })
            .collect();
        html! { <span>{courses}</span> }
    }
}
