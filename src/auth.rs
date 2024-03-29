use chrono::{Duration, Utc};
use leptos::logging::log;
use leptos::ServerFnError::*;
use leptos::*;
use leptos_actix::ResponseOptions;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

use crate::models::member;
use crate::state::AppState;
pub use actix_web::error::ErrorUnauthorized;
pub use actix_web::{dev::Payload, Error as ActixWebError};
pub use actix_web::{http, web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    pub user_data: member::Member,
    // gh_token: String, // used to access github and check user details
}

pub async fn auth(req: HttpRequest) -> Result<member::Member, ServerFnError> {
    req.cookie("auth_token")
        .ok_or(MissingArg("auth_token".into()))
        .map(|ck| ck.value().to_string())
        .and_then(|ok| Claims::decode(ok).map_err(Deserialization))
        .map(|token| token.claims.user_data)
}

#[cfg(feature = "ssr")]
impl Claims {
    pub fn decode(token: String) -> Result<TokenData<Claims>, String> {
        use crate::AUTH_SECRET;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(AUTH_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_err| "Could not decode token".into())
    }

    // mints a new token with expiry set for 30 days
    pub fn new(user_data: member::Member) -> Result<String, String> {
        use crate::AUTH_SECRET;
        let iat: usize = Utc::now().timestamp().try_into().unwrap();
        let exp: usize = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("Invalid Timestamp")
            .timestamp()
            .try_into()
            .unwrap();
        let claims = Claims {
            sub: user_data.id.to_string(), // user id
            iat,
            exp,
            user_data,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(AUTH_SECRET.as_bytes()),
        )
        .map_err(|_| "Could not generate a token".into())
    }
}

#[derive(Deserialize)]
struct ReqParams {
    code: String,
}

#[server(SignupCallback, "/api", "GetJson", "callback")]
pub async fn signup_callback() -> Result<String, ServerFnError> {
    use actix_web::web::Query;
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
    };
    use futures_util::FutureExt;
    use leptos_actix::extract;
    use leptos_actix::ResponseOptions;
    let srv_err = |err: String| ServerFnError::ServerError(err);
    let ua_token = extract(|search: Query<ReqParams>| async move { search.code.to_owned() })
        .then(|code| async move { exchange_code(code.unwrap_or("BAD".to_string())).await })
        .await
        .map_err(srv_err)?;
    log!("got token {ua_token}");
    let (gh_id, username, av_url) = user_info(ua_token).await.map_err(srv_err)?; // (github id, username, avatar url)
    log!("got user info {gh_id}");
    let user = member::Member::patch(gh_id, username, av_url)
        .await
        .map_err(srv_err)?;
    // generate token
    let token = Claims::new(user).map_err(srv_err)?;
    // attach cookie to response
    let response = expect_context::<ResponseOptions>();
    let cookie = Cookie::build("auth_token", token)
        .path("/")
        .same_site(SameSite::Lax)
        // .secure(true)
        .http_only(true)
        .finish()
        .to_string();
    response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    // happy days?
    leptos_actix::redirect("/");
    Ok("Registering User".to_string())
}

#[server(SignOut, "/api", "Url", "signout")]
pub async fn sign_out() -> Result<(), ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
    };
    let response = expect_context::<ResponseOptions>();
    let cookie = Cookie::build("auth_token", "")
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish()
        .to_string();
    response.insert_header(header::SET_COOKIE, HeaderValue::from_str(&cookie)?);
    Ok(())
}

#[cfg(feature = "ssr")]
/// exchanges callback code for an access token to github data
async fn exchange_code(code: String) -> Result<String, String> {
    use crate::{CLIENT_ID, CLIENT_SECRET, RQ};
    use itertools::Itertools;
    let params = [
        ("client_id", CLIENT_ID.to_owned()),
        ("client_secret", CLIENT_SECRET.to_owned()),
        ("code", code),
    ];
    let response = RQ
        .lock() // get reqwest client
        .map_err(|_| "Could not get request client")?
        .post("https://github.com/login/oauth/access_token")
        .form(&params)
        .send() // send post req to github/login/oauth/access_token
        .await
        .map_err(|err| format!("Bad Response from github:{}", err))? // unprocessed response
        .text() // get response body
        .await
        .map_err(|err| format!("Bad Response from github:{}", err))?; // url encoded response body

    let token = response
        .split('&')
        .find_map(|kv_pair| {
            kv_pair
                .splitn(2, '=')
                .map(|x| x.to_string())
                .collect_tuple()
                .filter(|(key, _value)| key == "access_token")
                .map(|(_key, value)| value)
        })
        .unwrap_or("Couldn't parse the response from github".into()); // get first kv where key == "access_token"
    log!("Value: {token}");
    // send rto
    Ok(token)
}

#[cfg(feature = "ssr")]
async fn user_info(token: String) -> Result<(i32, String, String), String> {
    use crate::{models::member::Member, CLIENT_ID, CLIENT_SECRET, RQ};
    use serde_json::json;
    let request = RQ
        .lock()
        .map_err(|_| "Could not lock req client")?
        .get("https://api.github.com/user")
        .header("Accept", "application/json")
        .header("User-Agent", "leptos/0.5")
        .header("Content-Type", "application/json")
        .bearer_auth(token.clone())
        .json(&json!({
            "access_token": token
        }));

    request
        .send()
        .await
        .and_then(|res| res.error_for_status())
        .map_err(|err| match err.status() {
            Some(code) => String::from(code.as_str()),
            None => "Something went wrong connecting to github".to_string(),
        })?
        .text()
        .await
        .map_err(|_| "Couldn't extract message contents".into())
        .and_then(|unparsed_text| {
            log!("{}", unparsed_text);
            serde_json::from_str::<serde_json::Value>(unparsed_text.as_str())
                .and_then(|json| {
                    Ok((
                        serde_json::from_value::<i32>(json["id"].clone())?,
                        serde_json::from_value::<String>(json["login"].clone())?,
                        serde_json::from_value::<String>(json["avatar_url"].clone())?,
                    ))
                })
                .map_err(|_| "Couldn't parse Json data".to_string())
        })
}
