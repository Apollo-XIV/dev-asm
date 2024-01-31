use crate::components::panel::Panel as PanelTemplate;
use crate::models::comment;
use crate::utils::time_since;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Panel(data: comment::Comment) -> impl IntoView {
    view! {
        <div>
            <PanelTemplate
                title={data.author}
                caption={format!("{} | {} ago",
                    data.date.format("%d/%m/%y"),
                    time_since(data.date))
            }>
                <p>{data.message}</p>
            </PanelTemplate>
        </div>
    }
}

#[component]
pub fn New(source: i32, alert: WriteSignal<bool>) -> impl IntoView {
    let post_comment = create_server_action::<comment::NewComment>();
    create_effect(move |_| {
        post_comment.version().get();
        alert.update(|value| *value = !*value);
    });
    view! {
        <PanelTemplate
            class="w-full"
            title={format!("Commenting as {}", 1)}>
            <ActionForm action=post_comment>
                <input type="hidden" name="thread_id" value={source} />
                <input type="hidden" name="author_id" value="1" />
                <textarea
                    name="message"
                    rows=2
                    placeholder="Write your comment here..."
                    class="focus:outline-none bg-transparent w-full p-2 text-wrap"
                    prop:value=move|| {post_comment.version().get(); ""}/>
                <button
                    type="submit"
                    class="w-full max-w-24 text-blue-950 float-right bg-amber-300 h-8 rounded-sm">
                    "reply"
                </button>
                <svg
                    aria-hidden="true"
                    class="w-8 h-8 text-gray-200 animate-spin dark:text-gray-600 fill-blue-600" viewBox="0 0 100 101" fill="none" xmlns="http://www.w3.org/2000/svg"
                    class:hidden=move || !post_comment.pending().get() >
                    <path d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z" fill="currentColor"/>
                    <path d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z" fill="currentFill"/>
                </svg>
            </ActionForm>

        </PanelTemplate>
    }
}

#[component]
pub fn Thread(id: i32, refetch: ReadSignal<bool>) -> impl IntoView {
    let comments = create_blocking_resource(move || (), move |_| comment::get_by_thread_id(id));
    create_effect(move |_| {
        refetch.get();
        comments.refetch()
    });
    view! {
        <Transition fallback=||()>
            <ErrorBoundary fallback=|_err|view!{<div class="w-12 h-12 bg-green-500"/>} >
                {move || comments().and_then(|resulted| Some(resulted.and_then(|comments| Ok(view!{
                    <ThreadView data=comments />
                }))))}
            </ErrorBoundary>
        </Transition>
    }
}

#[component]
fn ThreadView(data: Vec<comment::Comment>) -> impl IntoView {
    view! {
        <div class="flex flex-col w-full gap-2">
        {data.iter().cloned().map(|comment| view!{
            <Panel data=comment />
        }).collect_view()}
        </div>
    }
}
#[component]
fn Error() -> impl IntoView {
    view! {<p>"An error occurred"</p>}
}
