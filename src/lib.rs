pub mod app;
#[cfg(feature = "ssr")]
pub mod auth;
pub mod components;
pub mod models;
pub mod routes;
// #[cfg(feature = "ssr")]
pub mod state;
pub mod utils;
use cfg_if::cfg_if;
#[cfg(feature = "ssr")]
pub mod database;

cfg_if! {
if #[cfg(feature = "hydrate")] {

  use wasm_bindgen::prelude::wasm_bindgen;

    #[wasm_bindgen]
    pub fn hydrate() {
      use app::*;
      use leptos::*;

      console_error_panic_hook::set_once();

      leptos::mount_to_body(App);
    }
}
}

const DATE_FORMAT: &str = "%d/%m/%Y %H:%M";
use once_cell::sync::Lazy;
use reqwest::Client;
use std::sync::Mutex;
pub static RQ: Lazy<Mutex<Client>> = Lazy::new(|| Mutex::new(Client::new()));

#[cfg(feature = "ssr")]
use lazy_static::lazy_static;
#[cfg(feature = "ssr")]
lazy_static! {
    pub static ref CLIENT_ID: String = std::env::var("GITHUB_CLIENT_ID").unwrap();
    pub static ref CLIENT_SECRET: String = std::env::var("GITHUB_CLIENT_SECRET").unwrap();
    pub static ref AUTH_SECRET: String = std::env::var("AUTH_SECRET").unwrap();
}
