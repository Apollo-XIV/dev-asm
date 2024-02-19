use crate::models::member::Member;
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
}

#[server(TryAuth)]
pub async fn try_auth() -> Result<Member, ServerFnError> {
    use actix_web::HttpRequest;
    use leptos::ServerFnError::*;
    use leptos_actix::extract;
    let token = extract(|req: HttpRequest| async move { req })
        .await
        .and_then(|req| {
            req.cookie("auth_token")
                .ok_or(MissingArg("auth_token".into()))
        })
        .map(|ck| ck.value().to_string())?;
    let claims = crate::auth::Claims::decode(token)
        .map_err(Deserialization)?
        .claims;
    Ok(claims.user_data)
}

#[server(SignIn)]
pub async fn sign_in() -> Result<String, ServerFnError> {
    Ok(format!(
        "https://github.com/login/oauth/authorize?client_id={}",
        crate::CLIENT_ID.to_owned()
    ))
}

#[server(SignOut, "/api", "Url", "signout")]
pub async fn sign_out() -> Result<(), ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
    };
    use leptos_actix::ResponseOptions;
    let response = expect_context::<ResponseOptions>();
    let cookie = Cookie::build("auth_token", "")
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish()
        .to_string();
    response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    leptos_actix::redirect("/");
    Ok(())
}
#[derive(Debug, Clone)]
pub struct AuthCtx(pub Signal<Option<Member>>);

impl AuthCtx {
    // pub fn new(member: Member) -> Self {

    //     AuthCtx(member)
    // }
    pub fn get(&self) -> Option<Member> {
        self.0.get_untracked()
    }
}

// impl Default for AuthCtx {
//     fn default() -> Self {
//         AuthCtx(None)
//     }
// }
