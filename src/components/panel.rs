use leptos::*;

#[component]
pub fn Panel(
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] caption: TextProp,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!("p-1 bg-amber-300 text-white rounded-sm flex flex-col {}", class.get())>
            <div class="flex justify-between">
                <h1 class="pl-2 text-lg text-blue-950 font-bold italic">{title}</h1>
                <p class="text-blue-950 italic text-md">{caption}</p>
            </div>
            <div class="p-2 bg-slate-900 rounded-sm grow">{children()}</div>
        </div>
    }
}
