use crate::components::thread_view;
use crate::models::{comment, member, thread};
use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <a href="/forum/new" class="ml-auto p-2 bg-amber-300 rounded-sm">
            "New +"
        </a>
        <Await future=|| thread::get_all() let:data>
            <div class="w-full flex flex-col gap-2">
                {match data {
                    Ok(threads) => {
                        threads
                            .iter()
                            .map(|thread: &thread::Thread| {
                                view! {
                                    <thread_view::Thread data=thread
                                        .clone()></thread_view::Thread>
                                }
                            })
                            .collect_view()
                    }
                    Err(err) => view! { <p>{err.to_string()}</p> }.into_view(),
                }}

            </div>
        </Await>
    }
}
