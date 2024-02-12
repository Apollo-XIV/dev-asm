use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

// #[cfg(feature = "ssr")]
// #[derive(Serialize, Deserialize, Clone, Debug)]
// struct User {
//     id: u32,
// }
cfg_if! {
    if #[cfg(feature="ssr")] {
// #[cfg(feature = "ssr")]
    use actix_files::Files;
    // use actix_jwt_auth_middleware::use_jwt::UseJWTOnApp;
    // use actix_session::storage::CookieSessionStore;
    // use actix_session::SessionMiddleware;
    use actix_web::cookie::Key;
    use actix_web::HttpRequest;
    use actix_web::web::Path;
    use actix_web::http::header::HeaderMap;
    use actix_web::HttpResponse;
    use actix_web::*;
    use actix_web::web::{get, post};
    use finite_humour::app::*;
    use finite_humour::state::AppState;
    use finite_humour::AUTH_SECRET;
    use leptos::*;
    use leptos_actix::{generate_route_list, LeptosRoutes};

async fn server_fn_handler(
    path: Path<String>,
    // headers: HeaderMap,
    raw_query: RawQuery,
    token: finite_humour::auth::JwtAuth,
    request: HttpRequest
) -> impl Responder {
    use leptos::provide_context;
    use leptos_actix::handle_server_fns_with_context;
    handle_server_fns_with_context(path,  raw_query, move |_| {
        provide_context(token.clone())
    }, request)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let conf = get_configuration(None).await.unwrap();
    finite_humour::database::init_db()
        .await
        .expect("problem connecting to db");
    let addr = conf.leptos_options.site_addr;
    // Generate the list of routes in your Leptos App
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);
    println!("listening on http://{}", &addr);

    let app_state = AppState {
        leptos_options: leptos_options.clone(),
    };

    HttpServer::new(move || {
        // let leptos_options = &conf.leptos_options;
        let site_root = &leptos_options.site_root;

        App::new()
            .route(
                "/api/{tail:.*}", get().to(server_fn_handler).route(post().to(server_fn_handler))
            )
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", site_root))
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes_with_context(
                leptos_options.to_owned(),
                routes.to_owned(),
                || provide_context("testData"),
                App,
            )
            .app_data(web::Data::new(leptos_options.to_owned()))
        // .use_jwt(authority, web::scope("/hello").service(hello))
        // .wrap(
        //     SessionMiddleware::builder(
        //         CookieSessionStore::default(),
        //         Key::from(AUTH_SECRET.as_bytes()),
        //     )
        //     .cookie_secure(false)
        //     .build(),
        // )
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
}}
// #[cfg(feature = "ssr")]
// #[actix_web::get("/hello")]
// async fn hello(user: User) -> impl actix_web::Responder {
//     format!("Hello there {}", user.id)
// }

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use finite_humour::app::*;
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}
