use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod home;

use about::About;
use home::Home;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/haus/about")]
    About,
    #[not_found]
    #[at("/haus/page-not-found")]
    PageNotFound,
    #[at("/haus")]
    Home,
}

/// Switch app routes
pub fn switch(routes: &AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Home => html! { <Home /> },
        AppRoute::About => html! { <About /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}
