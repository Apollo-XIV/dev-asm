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
    let req = expect_context::<HttpRequest>()
        .cookie("auth_token")
        .ok_or_else(|| ServerFnError::ServerError("Could not find auth token".into()))?
        .value()
        .to_owned();
    log!("found cookie: {req:?}");
    Ok(AuthState { user: req })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    pub user: String,
}
#[derive(Debug, Clone, Copy)]
pub struct JwtAuth {
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Copy, Clone)]
pub struct AuthCtx(pub Signal<Option<crate::state::AuthState>>);

impl AuthCtx {
    pub fn get(&self) -> Option<AuthState> {
        self.0.get()
    }
}

impl Default for AuthCtx {
    fn default() -> Self {
        AuthCtx(Signal::derive(move || None))
    }
}
