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
                <Panel title="Account".to_string()>
                    <p class="text-sm">"Signed in as:"</p>
                    <h1 class="text-lg font-semibold">"TestUser"</h1>
                    <div class="flex justify-between text-sm italic">
                        <p>"acct. rep"</p>
                        <p>"acct. age"</p>
                    </div>
                    <div class="flex justify-between text-sm underline">
                        <a href="/account/board">"My Board"</a>
                        <a href="/sign-out">"Sign Out"</a>
                    </div>
                </Panel>
            </div>
        </nav>
    }
}
