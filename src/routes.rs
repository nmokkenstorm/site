use crate::pages::{Blog, Home};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Blog => html! { <Blog/>},
        Route::Home => html! { <Home/>},
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
