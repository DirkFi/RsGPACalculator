//src/pages/home.rs
use std::cmp::min;
use wasm_bindgen::JsValue;
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
struct Course {
    id: usize,
    name: String,
    teacher: String,
    description: String,
    image: String,
    unit: i32,
}

struct State {
    courses: Vec<Course>,
    grades: Vec<f32>,
}

pub struct Home {
    state: State,
    checks: Vec<bool>,
}

pub enum Msg {
    UpdateValue(usize, String),
    Chosen(usize),
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
        let courses = vec![
            Course {
                id: 0,
                name: "Machine Learning".to_string(),
                teacher: "Steven Bergner".to_string(),
                description: "Machine learning is the study of computer algorithms that improve automatically through experience, which play an increasingly important role in artificial intelligence, computer science and beyond. The goal of this course is to introduce students to machine learning, starting from the foundations and gradually building up to modern techniques. Students in the course will learn about the theoretical underpinnings, modern applications and software tools for applying deep learning. This course is intended to be an introductory course for students interested in conducting research in machine learning or applying machine learning, and should prepare students for more advanced courses, such as CMPT 727 and CMPT 728. No previous knowledge of machine learning is assumed, but students are expected to have solid background in calculus, linear algebra, probability and programming using Python.".to_string(),
                image: "".to_string(),
                unit: 3,
            },
            Course{
                id: 1,
                name: "Big Data Lab I".to_string(),
                teacher: "Greg Baker".to_string(),
                description: "This course is one of two lab courses that are part of the Professional Master’s Program in Big Data in the School of Computing Science. This lab course aims to provide students with the hands-on experience needed for a successful career in Big Data in the information technology industry. Many of the assignments will be completed on massive publically available data sets giving them appropriate experience with cloud computing and the algorithms and software tools needed to master programming for Big Data. Over 13 weeks of lab work and 12 hours per week of lab time, the students will obtain a solid background in programming for Big Data.".to_string(),
                image: "".to_string(),
                unit: 6,
            }
        ];
        let grades: Vec<f32> = vec![0.0; courses.len()];
        let checks: Vec<bool> = vec![false; courses.len()];
        Self {
            state: State { courses, grades },
            checks,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
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
        html! {
            <div>
                <span>{courses}</span>
                <button>{"Generate"}</button>
                <p>{self.calculate_gpa()} </p>
            </div>
        }
    }
}
