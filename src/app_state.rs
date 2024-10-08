// app_state.rs
use crate::types::Course;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub courses: Rc<Vec<Course>>,
    pub grades: Rc<Vec<f32>>,
    pub checks: Rc<Vec<bool>>,
    pub user_courses: Rc<Vec<Course>>, // User-added courses
    pub user_grades: Rc<Vec<f32>>,
    pub user_checks: Rc<Vec<bool>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            courses: Rc::new(vec![]),
            grades: Rc::new(vec![]),
            checks: Rc::new(vec![]),
            user_courses: Rc::new(vec![]),
            user_grades: Rc::new(vec![]),
            user_checks: Rc::new(vec![]),
        }
    }
}

pub enum AppStateAction {
    UpdateAll {
        courses: Rc<Vec<Course>>,
        grades: Rc<Vec<f32>>,
        checks: Rc<Vec<bool>>,
        user_courses: Rc<Vec<Course>>,
        user_grades: Rc<Vec<f32>>,
        user_checks: Rc<Vec<bool>>,
    },
    UpdateAllNonUser {
        courses: Rc<Vec<Course>>,
        grades: Rc<Vec<f32>>,
        checks: Rc<Vec<bool>>,
    },
    UpdateAllUser {
        user_courses: Rc<Vec<Course>>,
        user_grades: Rc<Vec<f32>>,
        user_checks: Rc<Vec<bool>>,
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
                user_courses,
                user_grades,
                user_checks,
            } => Rc::new(Self {
                courses,
                grades,
                checks,
                user_courses,
                user_grades,
                user_checks,
            }),

            AppStateAction::UpdateAllNonUser {
                courses,
                grades,
                checks,
            } => Rc::new(Self {
                courses,
                grades,
                checks,
                user_courses: Rc::clone(&self.user_courses),
                user_grades: Rc::clone(&self.user_grades),
                user_checks: Rc::clone(&self.user_checks),
            }),
            AppStateAction::UpdateAllUser {
                user_courses,
                user_grades,
                user_checks,
            } => Rc::new(Self {
                courses: Rc::clone(&self.courses),
                grades: Rc::clone(&self.grades),
                checks: Rc::clone(&self.checks),
                user_courses,
                user_grades,
                user_checks,
            }),
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;
