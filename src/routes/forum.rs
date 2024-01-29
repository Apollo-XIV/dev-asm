use crate::components::thread_view;
use crate::models::{comment, member, thread};
use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <Await future=|| thread::get_all() let:data>
        <div class="w-full flex flex-col gap-2">
            {match data {
                    Ok(threads) => threads.iter()
                    .map(|thread: &thread::Thread| view!{
                        <thread_view::Thread data=thread.clone() />
                    }).collect_view(),
                    Err(err) => view!{<p>{err.to_string()}</p>}.into_view()
            }}
        </div>
        </Await>
    }
}
