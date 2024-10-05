// src/app.rs
use crate::app_state::{AppState, AppStateContext};
use crate::components::GPAOverview;
use crate::pages::{CourseDetail, Home};
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)] // Annotate as a function component
pub fn app() -> Html {
    // Now you can use hooks inside this function component
    let app_state = use_reducer(AppState::default);

    html! {
        <ContextProvider<AppStateContext> context={app_state.clone()}>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<AppStateContext>>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<Home/>},
        Route::CourseDetail { id } => {
            html! {<CourseDetail id={ id }/>}
        }
        Route::GPAView => {
            html! {
                <GPAOverview /> // Let GPAOverview handle context itself
            }
        }
    }
}
