use crate::components::panel;
use crate::state::AuthCtx;
use crate::state::SignOut;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Active() -> impl IntoView {
    let auth = expect_context::<AuthCtx>();
    let sign_out = create_server_action::<SignOut>();
    view! {
        <panel::Panel title="Signed in as">
        <Suspense fallback=||view!{<p>"Not Signed In"</p>}>
            {dbg!(auth.0.get().flatten().map(|some| view!{move || some.username}))}
            <ActionForm action=sign_out>
                <input value="sign out" type="submit"/>
            </ActionForm>
        </Suspense>
        </panel::Panel>
    }
}
