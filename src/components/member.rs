use crate::components::panel;
use crate::state::AuthCtx;
use crate::state::SignOut;
use crate::utils::get_client_id;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Active() -> impl IntoView {
    let auth = expect_context::<AuthCtx>();
    let sign_out = create_server_action::<SignOut>();
    let copy = auth.clone();
    view! {
        <panel::Panel title="Signed in as">
        <Show
            when=move || copy.get().is_some()
            fallback=||view!{<SignIn />} >
            {auth.get().map(move |some| view!{{move || some.clone().username}})}
            <ActionForm action=sign_out>
                <input class="cursor-pointer underline" value="sign out" type="submit"/>
            </ActionForm>
        </Show>
        </panel::Panel>
    }
}

#[component]
fn SignIn() -> impl IntoView {
    view! {
        <p>"Not Signed In"</p>
        <SignInLink />
    }
}

#[component]
pub fn SignInLink() -> impl IntoView {
    use crate::state::SignIn;
    let sign_in = create_server_action::<SignIn>();
    create_effect(move |_| {
        if let Some(Ok(redirect)) = sign_in.value().get() {
            window().location().set_href(&redirect).unwrap()
        }
    });
    view! {
        <button on:click=move|_| sign_in.dispatch(SignIn{})>"Sign in with github"</button>
    }
}
