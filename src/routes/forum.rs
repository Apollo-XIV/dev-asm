use crate::components::thread_view;
use crate::models::{comment, member, thread};
use leptos::*;

#[component]
pub fn ForumPage() -> impl IntoView {
    view! {
        <Await future=|| thread::get_all() let:data>
            {match data {
                    Ok(threads) => threads.iter()
                    .map(|thread: &thread::Thread| view!{
                        <thread_view::Thread data=thread.clone() />
                    }).collect_view(),
                    Err(err) => view!{<p>{err.to_string()}</p>}.into_view()
            }}
        </Await>
    }
}
