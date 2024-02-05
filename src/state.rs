use leptos::LeptosOptions;
#[derive(Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub routes: Vec<leptos_router::RouteListing>,
}
