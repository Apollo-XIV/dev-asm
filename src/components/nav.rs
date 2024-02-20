use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="h-screen hidden sm:flex lg:w-1/3 justify-center place-items-center flex-col p-2">
            <div class="ml-auto min-w-48 max-w-64 flex flex-col gap-2">
                <Panel title="Dev-ASM".to_string()>
                    <p class="mt-0 italic leading-tight text-sm">
                        "A community for low-level learning."
                    </p>
                    <ul class="text-lg mt-2">
                        <li class="hover:text-orange-400">
                            <a href="/">"/home"</a>
                        </li>
                        <li class="hover:text-orange-400">
                            <a href="/forum">"/forum"</a>
                        </li>
                        <li class="hover:text-orange-400">
                            <a href="/knowledge-base">"/knowledge-base"</a>
                        </li>
                        <li class="hover:text-orange-400">
                            <a href="/courses">"/courses"</a>
                        </li>
                    </ul>
                </Panel>
                <crate::components::member::Active />
            </div>
        </nav>
    }
}

/// Big Text variant of hte panel componentll
#[component]
pub fn Panel(
    #[prop(optional, into)] class: TextProp,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] caption: TextProp,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=format!(
            "p-1 bg-amber-300 text-white rounded-sm flex flex-col {}",
            class.get(),
        )>
            <div class="flex justify-between">
                <h1 class="pl-2 text-2xl leading-loose text-blue-950 font-bold italic">
                    {title}
                </h1>
                <p class="text-blue-950 italic text-md">{caption}</p>
            </div>
            <div class="p-2 bg-slate-900 rounded-sm grow">{children()}</div>
        </div>
    }
}
