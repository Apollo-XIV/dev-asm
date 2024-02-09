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
    let test_cookie = create_server_action::<TestCookie>();
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
                    <Panel title="fetch cookie test">
                        <button on:click=move|_|test_cookie.dispatch(TestCookie{}) >"test my cookie"</button>
                    </Panel>
                // <p>"logged in as:"{move || session.map(|rsc| rsc.name())}</p>
            </ErrorBoundary>
        </Suspense>
    }
}

#[server(TestCookie)]
async fn test_cookie() -> Result<(), ServerFnError> {
    use actix_session::Session;
    use leptos_actix::extract;
    let _cookie = extract(|session: Session| async move { dbg!(session.entries().to_owned()) })
        .await
        .map_err(|_| ServerFnError::ServerError("No Auth Cookie :()".into()))?;
    Ok(())
    // .then(|code| async move { exchange_code(code.unwrap_or("BAD".to_string())).await })
}

#[server(GrabCookie)]
async fn grab_cookie() -> Result<(), ServerFnError> {
    use actix_session::Session;
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
        HttpRequest,
    };
    use leptos_actix::extract;
    let _cookie =
        extract(|session: Session| async move { dbg!(session.insert("auth_token", "value")) })
            .await
            .map_err(|_| ServerFnError::ServerError("No Auth Cookie :()".into()))?;
    // use leptos_actix::ResponseOptions;
    // let response = expect_context::<ResponseOptions>();
    // let mut cookie = Cookie::build("auth_token", "Demo Value")
    //     // .same_site(SameSite::Strict)
    //     // .secure(true)
    //     // .http_only(true)
    //     .finish()
    //     .to_string();
    // response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    Ok(())
}
