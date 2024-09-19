//src/route.rs
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/course/{id}")]
    CourseDetail { id: usize },
    #[at("/")]
    HomePage,
}
