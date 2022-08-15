use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::nav::Nav;
use crate::components::auth::Auth;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Nav />
            <Auth />
            <Switch<AppRoute> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}
