use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

impl AuthState {
    pub fn name(&self) -> String {
        self.user.clone()
    }
}

#[server(TryAuth)]
pub async fn try_auth() -> Result<AuthState, ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
        HttpRequest,
    };
    // log!("I'm running!!!!");
    use chrono::DateTime;
    use leptos_actix::ResponseOptions;
    let req = dbg!(expect_context::<HttpRequest>().cookies())
        // .cookie("auth_token")
        .map_err(|_| ServerFnError::ServerError(dbg!("Could not find auth token").to_string()))?
        // .value()
        .to_owned();
    // log!("I ran {req:?}");
    Ok(AuthState {
        user: "test".to_string(),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    user: String,
}
