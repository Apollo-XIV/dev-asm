use crate::components::panel::Panel;
use crate::state::AuthState;
use crate::utils;
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Page() -> impl IntoView {
    let client_id = create_blocking_resource(|| (), move |_| utils::get_client_id());
    let grab_cookie = create_server_action::<GrabCookie>();
    let session = use_context::<AuthState>();
    view! {
        <Suspense fallback=||()>
            <ErrorBoundary fallback=|_err| view!{<p>"Error"</p>} >
                {move|| client_id.get().map(move|result| result.map(move|ok| view!{
                    <h1 class="text-xl font-bold text-white">"test page"</h1>
                    <Panel title="sign in test">
                        <a href=move||format!("https://github.com/login/oauth/authorize?client_id={}", ok)>
                            "Sign in with github"
                        </a>
                    </Panel>
                }))}
                    <Panel title="cookie test">
                        <button on:click=move|_|grab_cookie.dispatch(GrabCookie{}) >"gimmie a cookie"</button>
                    </Panel>
                // <p>"logged in as:"{move || session.map(|rsc| rsc.name())}</p>
            </ErrorBoundary>
        </Suspense>
    }
}

#[server(GrabCookie)]
async fn grab_cookie() -> Result<(), ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
        HttpRequest,
    };
    use leptos_actix::ResponseOptions;
    let response = expect_context::<ResponseOptions>();
    let mut cookie = Cookie::build("auth_token", "Demo Value")
        // .same_site(SameSite::Strict)
        // .secure(true)
        // .http_only(true)
        .finish()
        .to_string();
    response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    Ok(())
}
