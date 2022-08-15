use yew::prelude::*;
use yew::Callback;
use yew_oauth2::oauth2::*;
use yew_oauth2::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;

#[function_component(Auth)]
pub fn auth() -> Html {
    let login = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().start_login();
    });

    let logout = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().logout();
    });

    let config = Config {
        client_id: "frontend".into(),
        auth_url: "https://sso-ctron-drogue.apps.wonderful.iot-playground.org/auth/realms/Yew/protocol/openid-connect/auth".into(),
        token_url: "https://sso-ctron-drogue.apps.wonderful.iot-playground.org/auth/realms/Yew/protocol/openid-connect/token".into(),
    };

    html! {
        <OAuth2 {config}>
            <Failure><FailureMessage/></Failure>
            <Authenticated>
                <p><button onclick={logout}>{"Logout"}</button></p>
                <h1>{"Authenticated!"}</h1>
            </Authenticated>
            <NotAuthenticated>
                <p>
                    {"You need to log in"}
                    <button onclick={login.clone()}>
                        {"Login"}
                    </button>
                </p>
            </NotAuthenticated>
        </OAuth2>
    }
}
