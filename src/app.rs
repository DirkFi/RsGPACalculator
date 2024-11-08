// src/app.rs
use crate::app_state::{AppState, AppStateContext};
use crate::components::GPAOverview;
use crate::pages::{CourseDetail, Home};
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_reducer(AppState::default);

    html! {
        <>
            <ContextProvider<AppStateContext> context={app_state}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<AppStateContext>>
        </>
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
                <GPAOverview />
            }
        }
    }
}
