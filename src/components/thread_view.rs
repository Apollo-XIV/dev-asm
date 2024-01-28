use leptos::*;
use crate::models::thread;
use crate::components::panel::Panel;

#[component]
pub fn Thread(#[prop(into)] data: thread::Thread) -> impl IntoView {
    view! {
        <Panel title={data.author}>
            <p>{data.id}</p>
            <p>{data.title}</p>
        </Panel>
    }
}
