use leptos::*;

#[component]
pub fn Todo(#[prop(optional, into)] class: TextProp) -> impl IntoView {
    view! { <div class=class>"TODO"</div> }
}
