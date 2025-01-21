// src/app.rs
use crate::app_state::{AppState, AppStateContext};
use crate::components::GPAOverview;
use crate::pages::{CourseDetail, Home};
use crate::route::Route;
use web_sys::window;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    // Now you can use hooks inside this function component
    // Retrieve the initial theme from localStorage or default to "light"
    let initial_theme = {
        let window = window().expect("window should exist");

        // Safely handle the case where localStorage might be unavailable
        if let Some(storage) = window.local_storage().ok().flatten() {
            match storage.get_item("theme") {
                Ok(Some(theme)) => theme,
                _ => "light".to_string(), // Fallback to light theme if nothing found
            }
        } else {
            "light".to_string() // Fallback if localStorage is unavailable
        }
    };
    // Use local state to track the current theme
    let theme = use_state(|| initial_theme);

    // Function to toggle the theme between light and dark
    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            let new_theme = if *theme == "light" { "dark" } else { "light" };
            theme.set(new_theme.to_string());

            // Save the new theme to localStorage
            let window = window().unwrap();
            let storage = window.local_storage().unwrap().unwrap();
            storage.set_item("theme", &new_theme).unwrap();
        })
    };

    // Determine the current theme class
    let current_theme_class = if *theme == "light" { "light" } else { "dark" };
    let app_state = use_reducer(AppState::default);

    html! {
        <div class={current_theme_class}>
            <button onclick={toggle_theme}>
                { if *theme == "light" { "Switch to Dark Mode" } else { "Switch to Light Mode" } }
            </button>

            <ContextProvider<AppStateContext> context={app_state}>
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            </ContextProvider<AppStateContext>>
        </div>
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
