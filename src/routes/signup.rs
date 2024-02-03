use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn Page() -> impl IntoView {
    let client_id = create_blocking_resource(|| (), move |_| get_client_id());
    view! {
        <Suspense fallback=||()>
            <ErrorBoundary fallback=|_err| view!{<p>"Error"</p>} >
                {move|| client_id.get().map(move|result| result.map(move|ok| view!{
                    <h1>"test page"</h1>
                    <a href=move||format!("https://github.com/login/oauth/authorize?client_id={}", ok)>
                        "Sign in with github"
                    </a>
                }))}
            </ErrorBoundary>
        </Suspense>
    }
}

#[derive(Deserialize)]
struct ReqParams {
    code: String,
}

#[server(SignupCallback, "/api", "GetJson", "callback")]
pub async fn signup_callback() -> Result<String, ServerFnError> {
    use actix_web::dev::ConnectionInfo;
    use actix_web::web::{Data, Query};
    use futures_util::FutureExt;
    use leptos_actix::extract;
    let map_server_error = |err: &str| ServerFnError::ServerError(err.to_string());
    let request = extract(
        |search: Query<ReqParams>, connection: ConnectionInfo| async move {
            format!("search = {}\nconnection={:?}", search.code, connection)
        },
    )
    .await?;
    log!("Request: {request}");
    let ua_token = extract(|search: Query<ReqParams>| async move { search.code.to_owned() })
        .then(|code| async move { exchange_code(code.unwrap_or("BAD".to_string())).await })
        .await
        .map_err(map_server_error)?;
    log!("got token {ua_token}");
    let info = user_info(ua_token).await.map_err(map_server_error)?;
    log!("got user info {info}");
    leptos_actix::redirect("/");
    Ok("Registering User".to_string())
}

#[cfg(feature = "ssr")]
async fn exchange_code(code: String) -> Result<String, &'static str> {
    use crate::{CLIENT_ID, CLIENT_SECRET, RQ};
    use itertools::Itertools;
    use reqwest::multipart;
    let params = [
        ("client_id", CLIENT_ID.to_owned()),
        ("client_secret", CLIENT_SECRET.to_owned()),
        ("code", code),
    ];
    let (key, value) = RQ
        .lock() // get reqwest client
        .map_err(|_| "Could not get request client")?
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .send() // send post req to github/login/oauth/access_token
        .await
        .map_err(|_err| "Bad response from github. Please contact the administrator")?
        .text() // get response body
        .await
        .map_err(|_err| "Bad response from github. Please contact the administrator")? // url encoded response body
        .split('&')
        .find_map(|kv_pair| {
            kv_pair
                .splitn(2, '=')
                .map(|x| x.to_string())
                .collect_tuple()
                .filter(|(key, _value)| key == "access_token")
        }) // get first kv where key == "access_token"
        .ok_or("Could not extract body content")?;
    log!("Key: {key} Value: {value}");
    // send rto
    Ok(value)
}

use crate::models::member;
#[cfg(feature = "ssr")]
async fn user_info(token: String) -> Result<String, &'static str> {
    use crate::{CLIENT_ID, CLIENT_SECRET, RQ};
    use serde_json::json;
    RQ.lock()
        .map_err(|_| "Could not lock req client")?
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .header("User-Agent", "leptos/0.5")
        .header("Content-Type", "application/json")
        .bearer_auth(token.clone())
        .json(&json!({
            "access_token": token
        }))
        .send()
        .await
        .map_err(|_| "Bad Response from Github")?
        // .and_then(|res: reqwest::Response| match res.status().as_u16() {
        //     200 => Ok(res),
        //     _ => Err(res.text().await?),
        // })?
        .text()
        .await
        .map_err(|_| "Bad Response")
}

#[server]
pub async fn get_client_id() -> Result<String, ServerFnError> {
    use crate::CLIENT_ID;
    Ok(CLIENT_ID.to_owned())
}
