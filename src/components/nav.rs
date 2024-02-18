use super::panel::Panel;
use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="h-screen hidden sm:flex lg:w-1/3 justify-center place-items-center flex-col p-2">
            <div class="ml-auto min-w-48 max-w-64 flex flex-col gap-2">
                <Panel title="Placeholder".to_string()>
                    <p class="mt-0 italic leading-tight text-sm">
                        "A community for developers to ask just about anything."
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
