// src/components/gpa_overview.rs
use crate::pages::point_to_pa;
use crate::types::Course;
use std::cmp::min;
use yew::prelude::*;

pub struct GPAOverview {
    props: Props,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub courses: Vec<Course>,
    pub grades: Vec<f32>,
    pub checks: Vec<bool>,
}

impl GPAOverview {
    fn calculate_gpa(&self) -> f32 {
        let mut numer: f32 = 0.0;
        let mut denomi: f32 = 0.0;
        for i in 0..self.props.courses.len() {
            if i < min(self.props.checks.len(), self.props.grades.len()) && self.props.checks[i] {
                numer += point_to_pa(self.props.grades[i]) * self.props.courses[i].unit as f32;
                denomi += self.props.courses[i].unit as f32;
            }
        }
        numer / denomi
    }
}

impl Component for GPAOverview {
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
        let mut courses_view = vec![];
        for i in 0..self.props.courses.len() {
            let course = self.props.courses[i].clone();
            let grade = &self.props.grades[i];
            let check = self.props.checks[i];
            if check {
                courses_view.push(html! {
                <tr>
                    <td style="padding: 10px; text-align: center;">{&course.name}</td>
                    <td style="padding: 10px; text-align: center;">{grade}</td>
                    <td style="padding: 10px; text-align: center;">{&course.unit}</td>
                </tr>
                });
            }
        }
        html! {
            <div>
                <h2 style="text-align: center;">{"GPA Overview"}</h2>
                <table style="
                    width: 60%; 
                    border-collapse: collapse; 
                    margin: 20px auto; 
                    border: 1px solid #333;">
                    <tr style="background-color: #f2f2f2;">
                        <th style="padding: 10px; text-align: center;">{"Course Name"}</th>
                        <th style="padding: 10px; text-align: center;">{"Course Grade"}</th>
                        <th style="padding: 10px; text-align: center;">{"Unit"}</th>
                    </tr>
                    {courses_view}
                </table>
                <div style="font-size: 18px; font-weight: bold; text-align: center; margin-top: 20px;">
                    {format!("Overall GPA is：{}", self.calculate_gpa())}
                </div>
            </div>
        }
    }
}
