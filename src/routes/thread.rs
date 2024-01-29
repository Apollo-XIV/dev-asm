use crate::components::{comment_view, thread_view};
use crate::{
    components::panel::Panel,
    models::{comment, member, thread},
};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let post_comment = create_server_action::<comment::NewComment>();
    let thread_data = create_resource(
        move || {
            params
                .get()
                .get("id")
                .cloned()
                .unwrap_or_default()
                .parse()
                .unwrap()
        },
        |thread_id| async move { thread::get_by_id(thread_id).await },
    );
    view! {
        <Suspense fallback=||()>
            <ErrorBoundary fallback=|_err|() >
                {move || thread_data
                    .get()
                    .map(move|x|{
                        x.map(move|thread| view!{
                            <thread_view::Header data=thread />
                        })
                    })
                }
                <Panel
                    class="w-full"
                    title={format!("Commenting as {}", 1)}>
                    <ActionForm action=post_comment>
                        {move||thread_data.get().map(move|x| x.map(move|value| view!{
                            <input type="hidden" name="thread_id" value={value.id} />
                        }))}
                        <input type="hidden" name="author_id" value="1" />
                        <textarea
                            name="message"
                            rows=2
                            placeholder="Write your comment here..."
                            class="focus:outline-none bg-transparent w-full p-2 text-wrap"/>
                        <button
                            type="submit"
                            class="w-full max-w-24 text-blue-950 float-right bg-amber-300 h-8 rounded-sm">
                            "reply"
                        </button>
                    </ActionForm>
                </Panel>
            </ErrorBoundary>
        </Suspense>
        // <Await future=|| thread::get_by_id(id.clone()) let:data>
        //     <ErrorBoundary fallback=|_|() >
        //         <thread_view::Header data={data.clone().unwrap()} />
        //     </ErrorBoundary>

        //     <Await future=move || comment::get_by_thread_id(id.clone()) let:comments>
        //         {match comments {
        //             Ok(x) => view!{<ThreadView data=x.clone() />},
        //             Err(_x) => view!{<Error />}
        //         }}
        //     </Await>
        // </Await>
    }
}

#[component]
fn Error() -> impl IntoView {
    view! {<p>"An error occurred"</p>}
}

#[component]
fn ThreadView(data: Vec<comment::Comment>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-2">
        {data.iter().cloned().map(|comment| view!{
            <comment_view::Panel data=comment />
        }).collect_view()}
        </div>
    }
}
