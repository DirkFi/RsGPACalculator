// src/app.rs
use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{CourseDetail, Home};
use crate::route::Route;
use web_sys::console;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();
    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // let render = Router::render(|switch: Route| match switch {
        //     Route::HomePage => html! {<Home/>},
        // });
        //
        // html! {
        //     <BrowserRouter>
        //         <Switch<Route> render={Switch::render(render)} / >
        //     </BrowserRouter>
        // }
        html! {
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::HomePage => html! {<Home/>},
        Route::CourseDetail { id } => {
            console::log_1(&"Successfully get the Route::CourseDetail!".into());
            html! {<CourseDetail id={ id }/>}
        }
    }
}
