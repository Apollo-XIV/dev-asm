use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    sub: String,
    iat: usize,
    exp: usize,
    ua_token: String,
}

pub fn new_token() {}
