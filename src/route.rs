//src/route.rs
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    HomePage,
}
