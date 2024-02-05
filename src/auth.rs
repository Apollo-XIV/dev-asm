use crate::models::member::Member;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use leptos::logging::log;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    ua_token: String, // used to access github and check user details
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthState {
    user: String,
}

impl AuthState {
    pub fn name(&self) -> String {
        self.user.clone()
    }
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
    fn new(id: i32, ua_token: String) -> String {
        use crate::AUTH_SECRET;
        let iat: usize = Utc::now().timestamp().try_into().unwrap();
        let exp: usize = Utc::now()
            .checked_add_signed(Duration::days(30))
            .expect("Invalid Timestamp")
            .timestamp()
            .try_into()
            .unwrap();
        let claims = Claims {
            sub: id.to_string(), // user id
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

#[server(TryAuth)]
pub async fn try_auth() -> Result<AuthState, ServerFnError> {
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
        HttpRequest,
    };
    log!("I'm running!!!!");
    use chrono::DateTime;
    use leptos_actix::ResponseOptions;
    let req = dbg!(expect_context::<HttpRequest>().cookies())
        // .cookie("auth_token")
        .map_err(|_| ServerFnError::ServerError(dbg!("Could not find auth token").to_string()))?
        // .value()
        .to_owned();
    log!("I ran {req:?}");
    Ok(AuthState {
        user: "test".to_string(),
    })
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
