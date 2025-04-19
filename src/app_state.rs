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
    GradesLetter(Vec<String>),
    UserGradesLetter(Vec<String>),
}

#[derive(Clone, PartialEq)]
pub struct AppState {
    pub courses: Rc<Vec<Course>>,
    pub grades: Rc<Vec<f32>>,
    pub checks: Rc<Vec<bool>>,
    pub user_courses: Rc<Vec<Course>>, // User-added courses
    pub user_grades: Rc<Vec<f32>>,
    pub user_checks: Rc<Vec<bool>>,
    pub grades_letter: Rc<Vec<String>>,
    pub user_grades_letter: Rc<Vec<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            courses: Rc::new(vec![]),
            grades: Rc::new(vec![]),
            grades_letter: Rc::new(vec![]),
            checks: Rc::new(vec![]),
            user_courses: Rc::new(vec![]),
            user_grades: Rc::new(vec![]),
            user_grades_letter: Rc::new(vec![]),
            user_checks: Rc::new(vec![]),
        }
    }
}

pub enum AppStateAction {
    UpdateAll {
        courses: Rc<Vec<Course>>,
        grades: Rc<Vec<f32>>,
        grades_letter: Rc<Vec<String>>,
        checks: Rc<Vec<bool>>,
        user_courses: Rc<Vec<Course>>,
        user_grades: Rc<Vec<f32>>,
        user_grades_letter: Rc<Vec<String>>,
        user_checks: Rc<Vec<bool>>,
    },
    UpdateAllNonUser {
        courses: Rc<Vec<Course>>,
        grades: Rc<Vec<f32>>,
        grades_letter: Rc<Vec<String>>,
        checks: Rc<Vec<bool>>,
    },
    UpdateAllUser {
        user_courses: Rc<Vec<Course>>,
        user_grades: Rc<Vec<f32>>,
        user_grades_letter: Rc<Vec<String>>,
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
                grades_letter,
                checks,
                user_courses,
                user_grades,
                user_grades_letter,
                user_checks,
            } => Rc::new(Self {
                courses,
                grades,
                grades_letter,
                checks,
                user_courses,
                user_grades,
                user_grades_letter,
                user_checks,
            }),

            AppStateAction::UpdateAllNonUser {
                courses,
                grades,
                grades_letter,
                checks,
            } => Rc::new(Self {
                courses,
                grades,
                grades_letter,
                checks,
                user_courses: Rc::clone(&self.user_courses),
                user_grades: Rc::clone(&self.user_grades),
                user_grades_letter: Rc::clone(&self.user_grades_letter),
                user_checks: Rc::clone(&self.user_checks),
            }),
            AppStateAction::UpdateAllUser {
                user_courses,
                user_grades,
                user_grades_letter,
                user_checks,
            } => Rc::new(Self {
                courses: Rc::clone(&self.courses),
                grades: Rc::clone(&self.grades),
                grades_letter: Rc::clone(&self.grades_letter),
                checks: Rc::clone(&self.checks),
                user_courses,
                user_grades,
                user_grades_letter,
                user_checks,
            }),
            AppStateAction::UpdateSingle { values } => match values {
                AppStateValue::Courses(courses) => {
                    update_with_rc!(
                        self,
                        { grades, grades_letter, checks, user_courses, user_grades, user_grades_letter, user_checks },
                        { courses: Rc::new(courses) }
                    )
                }
                AppStateValue::Grades(grades) => {
                    update_with_rc!(
                        self,
                        { courses, grades_letter, checks, user_courses, user_grades, user_grades_letter, user_checks },
                        { grades: Rc::new(grades) }
                    )
                }
                AppStateValue::GradesLetter(grades_letter) => {
                    update_with_rc!(
                        self,
                        { courses, grades, checks, user_courses, user_grades, user_grades_letter, user_checks },
                        { grades_letter: Rc::new(grades_letter) }
                    )
                }
                AppStateValue::Checks(checks) => {
                    update_with_rc!(
                        self,
                        { courses, grades, grades_letter, user_courses, user_grades, user_grades_letter, user_checks },
                        { checks: Rc::new(checks) }
                    )
                }
                AppStateValue::UserCourses(user_courses) => {
                    update_with_rc!(
                        self,
                        { courses, grades, grades_letter, checks, user_grades, user_grades_letter, user_checks },
                        { user_courses: Rc::new(user_courses) }
                    )
                }
                AppStateValue::UserGrades(user_grades) => {
                    update_with_rc!(
                        self,
                        { courses, grades, grades_letter, checks, user_courses, user_grades_letter, user_checks },
                        { user_grades: Rc::new(user_grades) }
                    )
                }
                AppStateValue::UserGradesLetter(user_grades_letter) => {
                    update_with_rc!(
                        self,
                        { courses, grades, grades_letter, checks, user_courses, user_grades, user_checks },
                        { user_grades_letter: Rc::new(user_grades_letter) }
                    )
                }
                AppStateValue::UserChecks(user_checks) => {
                    update_with_rc!(
                        self,
                        { courses, grades, grades_letter, checks, user_courses, user_grades, user_grades_letter },
                        { user_checks: Rc::new(user_checks) }
                    )
                }
            },
        }
    }
}

pub type AppStateContext = UseReducerHandle<AppState>;
