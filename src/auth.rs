use chrono::{Duration, Utc};
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};

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
    ua_token: String, // used to access github and check user details
}

#[cfg(feature = "ssr")]
impl Claims {
    fn decode(token: String) -> Result<TokenData<Claims>, &'static str> {
        use crate::AUTH_SECRET;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(AUTH_SECRET.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_err| "Could not decode token")
    }

    // mints a new token with expiry set for 30 days
    pub fn new(id: String, ua_token: String) -> String {
        use crate::AUTH_SECRET;
        let iat: usize = Utc::now().timestamp().try_into().unwrap();
        let exp: usize = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("Invalid Timestamp")
            .timestamp()
            .try_into()
            .unwrap();
        let claims = Claims {
            sub: id, // user id
            iat,
            exp,
            ua_token,
        };
        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(AUTH_SECRET.as_bytes()),
        ) {
            Ok(x) => dbg!(x),
            Err(err) => panic!(),
        };
        token.to_string()
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

#[derive(Clone, Copy)]
pub struct JwtAuth {
    pub user_id: uuid::Uuid,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}
use std::fmt;

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl FromRequest for JwtAuth {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // let data = req.app_data::<web::Data<AppState>>().expect()();

        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            let json_error = ErrorResponse {
                status: "fail".to_string(),
                message: "You are not logged in, please provide a token".to_string(),
            };
            return ready(Err(ErrorUnauthorized(json_error)));
        };

        let claims = match Claims::decode(token.unwrap()) {
            Ok(x) => x.claims,
            Err(x) => {
                let json_error = ErrorResponse {
                    status: "fail".to_string(),
                    message: "You are not logged in, please provide a token".to_string(),
                };
                return ready(Err(ErrorUnauthorized(json_error)));
            }
        };
        log!("{:?}", claims);
        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());
        ready(Ok(JwtAuth { user_id }))
    }
}
