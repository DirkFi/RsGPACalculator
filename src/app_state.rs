// app_state.rs
use crate::types::Course;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub courses: Rc<Vec<Course>>,
    pub grades: Rc<Vec<f32>>,
    pub checks: Rc<Vec<bool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            courses: Rc::new(vec![]),
            grades: Rc::new(vec![]),
            checks: Rc::new(vec![]),
        }
    }
}

pub enum AppStateAction {
    UpdateAll {
        courses: Rc<Vec<Course>>,
        grades: Rc<Vec<f32>>,
        checks: Rc<Vec<bool>>,
    },
}

impl Reducible for AppState {
    type Action = AppStateAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            AppStateAction::UpdateAll {
                courses,
                grades,
                checks,
            } => Rc::new(Self {
                courses,
                grades,
                checks,
            }),
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;
