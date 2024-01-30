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
    let (refresh, set_refresh) = create_signal(false);
    view! {
        <Suspense fallback=||()>
            <ErrorBoundary fallback=|_err|() >
                {move || thread_data
                    .get()
                    .map(move|x|{
                        x.map(move|thread| view!{
                            <thread_view::Header data=thread.clone() />
                            <comment_view::Thread id=thread.clone().id refetch=refresh />
                            <comment_view::New source=thread.id alert=set_refresh />
                        })
                    })
                }
            </ErrorBoundary>
        </Suspense>
    }
}
