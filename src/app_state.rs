// app_state.rs
use crate::types::Course;
use std::rc::Rc;
use yew::prelude::*;

macro_rules! update_with_rc {
    ($self:ident, { $( $field:ident ),* }, { $( $non_rc_field:ident : $value:expr ),* }) => {
        Rc::new(Self {
            $(
                $field: Rc::clone(&$self.$field),
            )*
            $(
                $non_rc_field: $value,
            )*
        })
    };
}

#[derive(Clone, PartialEq)]
pub enum AppStateValue {
    Courses(Vec<Course>),
    Grades(Vec<f32>),
    Checks(Vec<bool>),
    UserCourses(Vec<Course>),
    UserGrades(Vec<f32>),
    UserChecks(Vec<bool>),
}

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

    UpdateSingle {
        values: AppStateValue,
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
            AppStateAction::UpdateSingle { values } => {
                match values {
                    AppStateValue::Courses(courses) => {
                        update_with_rc!(
                            self,
                            {  grades, checks,user_courses, user_grades,  user_checks },   // Rc::clone fields
                            { courses: Rc::new( courses ) }  // non-Rc fields
                        )
                    }
                    AppStateValue::Grades(grades) => {
                        update_with_rc!(
                            self,
                            {  courses, checks,user_courses, user_grades,  user_checks },   // Rc::clone fields
                            { grades: Rc::new( grades ) }  // non-Rc fields
                        )
                    }
                    AppStateValue::Checks(checks) => {
                        update_with_rc!(
                            self,
                            {  courses, grades,user_courses, user_grades,  user_checks },   // Rc::clone fields
                            { checks: Rc::new( checks ) }  // non-Rc fields
                        )
                    }
                    AppStateValue::UserCourses(user_courses) => {
                        update_with_rc!(
                            self,
                            {  courses, grades,checks, user_grades,  user_checks },   // Rc::clone fields
                            { user_courses: Rc::new( user_courses ) }  // non-Rc fields
                        )
                    }
                    AppStateValue::UserGrades(user_grades) => {
                        update_with_rc!(
                            self,
                            {  courses, grades,checks, user_courses,  user_checks },   // Rc::clone fields
                            { user_grades: Rc::new( user_grades ) }  // non-Rc fields
                        )
                    }
                    AppStateValue::UserChecks(user_checks) => {
                        update_with_rc!(
                            self,
                            {  courses, grades,checks, user_courses,  user_grades },   // Rc::clone fields
                            { user_checks: Rc::new( user_checks ) }  // non-Rc fields
                        )
                    }
                }
            }
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;
