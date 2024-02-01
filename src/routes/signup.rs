use leptos::*;

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

#[server]
pub async fn get_client_id() -> Result<String, ServerFnError> {
    std::env::var("GITHUB_CLIENT_ID")
        .map_err(|_err| ServerFnError::ServerError("Couldn't fetch OAuth Client ID".to_string()))
}
