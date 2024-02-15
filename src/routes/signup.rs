use crate::components::panel::Panel;
use crate::state::AuthCtx;
use crate::state::AuthState;
use crate::utils;
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Page() -> impl IntoView {
    let client_id = create_blocking_resource(|| (), move |_| utils::get_client_id());
    let grab_jwt = create_server_action::<GrabJwt>();
    let session = use_context::<AuthCtx>().unwrap_or_default();
    view! {
        <Suspense fallback=|| ()>
            <ErrorBoundary fallback=|_err| {
                view! { <p>"Error"</p> }
            }>
                {move || session.get().map(|val| val.user)}
                {move || {
                    client_id
                        .get()
                        .map(move |result| {
                            result
                                .map(move |ok| {
                                    view! {
                                        <h1 class="text-xl font-bold text-white">"test page"</h1>
                                        <Panel title="sign in test">
                                            <a href=move || {
                                                format!(
                                                    "https://github.com/login/oauth/authorize?client_id={}",
                                                    ok,
                                                )
                                            }>"Sign in with github"</a>
                                        </Panel>
                                    }
                                })
                        })
                }}
                <Panel title="cookie test">
                    <button on:click=move |_| {
                        grab_jwt.dispatch(GrabJwt {})
                    }>"gimmie a cookie"</button>
                </Panel> <Panel title="fetch cookie test">
                    <button on:click=move |_| ()>"test my cookie"</button>
                </Panel>
            // <p>"logged in as:"{move || session.map(|rsc| rsc.name())}</p>
            </ErrorBoundary>
        </Suspense>
    }
}

// #[server(TestCookie)]
// async fn test_cookie() -> Result<(), ServerFnError> {
//     use actix_session::Session;
//     use leptos_actix::extract;
//     let _cookie = extract(|session: Session| async move { dbg!(session.entries().to_owned()) })
//         .await
//         .map_err(|_| ServerFnError::ServerError("No Auth Cookie :()".into()))?;
//     Ok(())
//     // .then(|code| async move { exchange_code(code.unwrap_or("BAD".to_string())).await })
// }

#[server(GrabJwt)]
async fn grab_jwt() -> Result<(), ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
        HttpRequest,
    };
    use leptos_actix::extract;
    let _cookie =
        extract(|req: HttpRequest| async move { dbg!(req.cookie("auth_token").to_owned()) })
            .await
            .map_err(|_| ServerFnError::ServerError("No Auth Cookie :()".into()))?;
    use leptos_actix::ResponseOptions;
    let response = expect_context::<ResponseOptions>();
    let cookie = Cookie::build("auth_token", "Demo Value")
        .path("/")
        .same_site(SameSite::Lax)
        // .secure(true)
        .http_only(true)
        .finish()
        .to_string();
    response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    Ok(())
}

#[server(TestJwt, "/api", "Url", "test")]
async fn test_jwt() -> Result<String, ServerFnError> {
    use crate::state::JwtAuth;
    use leptos_actix::extract;
    let _jwt = extract(|token: JwtAuth| async move { dbg!(token.user_id) })
        .await
        .map_err(|_| ServerFnError::ServerError("Bad Req.".into()))?;
    Ok("It worked".into())
}

// #[server(GetJwt, "/api", "Url", "get_jwt")]
// async fn get_jwt() -> Result<String, ServerFnError> {
//     use crate::auth::Claims;
//     Ok(Claims::new(
//         "a1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8".into(),
//         "demo".into(),
//     ))
// }
