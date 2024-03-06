use crate::components::{nav::Nav, panel::Panel, todo::Todo};
use crate::routes::{forum, signup, thread};
use crate::state::try_auth;
use crate::state::AuthCtx;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // let session = create_blocking_resource(|| (), move |_| async { dbg!(try_auth().await.ok()) });
    // let derived = Signal::derive(move || {
    //     logging::log!("{:?}", session.get().flatten());
    //     session.get().flatten()
    // });
    // provide_context(AuthCtx(Signal::derive(move || None)));
    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-start.css"/>
        <Stylesheet id="leptos" href="/assets/tailwind.css"/>

        // Fira-Mono CDN Link from google-fonts
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
        <link
            href="https://fonts.googleapis.com/css2?family=Fira+Mono:wght@400;500;700&display=swap"
            rel="stylesheet"
        />

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <body class="bg-slate-900 flex place-items-center min-h-screen">
                <AuthProvider>
                    <Nav/>
                </AuthProvider>
                <main class="max-w-3xl max-h-screen overflow-y-scroll overflow-x-hidden h-fit flex flex-wrap justify-center place-items-stretch gap-2 w-full p-2">
                    <Routes>
                        <Route path="" view=|| view! { <Outlet/> } ssr=SsrMode::PartiallyBlocked>
                            // <Suspense fallback=|| view!{}>
                            // {move || session.get().map(|x|x.map(|y| "test"))}
                            // {move || {
                            // let derived = Signal::derive(move || {
                            // logging::log!("logged in as: {:?}", session.get().flatten());
                            // session.get().flatten()
                            // });
                            // provide_context(AuthCtx(derived));
                            // }}
                            // </Suspense>
                            <Route
                                path=""
                                view=|| {
                                    view! {
                                        <AuthProvider>
                                            <HomePage/>
                                        </AuthProvider>
                                    }
                                }
                            />

                            <Route
                                path="/forum"
                                view=|| {
                                    view! {
                                        <AuthProvider>
                                            <Outlet/>
                                        </AuthProvider>
                                    }
                                }
                            >

                                <Route path="/new" view=thread::New/>
                                <Route path=":id" view=thread::Page/>
                                <Route path="" view=forum::Page/>
                            </Route>
                            <Route
                                path="/signup"
                                view=|| {
                                    view! {
                                        <AuthProvider>
                                            <signup::Page></signup::Page>
                                        </AuthProvider>
                                    }
                                }
                            />

                            <Route path="/*any" view=NotFound/>
                        </Route>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

#[component]
fn AuthProvider(children: ChildrenFn) -> impl IntoView {
    let session = create_blocking_resource(|| (), move |_| async { try_auth().await.ok() });
    let derived = Signal::derive(move || {
        // logging::log!("logged in as: {:?}", session.get().flatten());
        session.get().flatten()
    });
    provide_context(AuthCtx(derived));
    view! { <Suspense fallback=|| ()>{children()}</Suspense> }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Stats/>
        <Activity/>
        <RecentThreads/>
    }
}

#[component]
fn Stats() -> impl IntoView {
    view! {
        <div class="basis-36 grow">
            <Panel title="Stats" class="h-full">
                <div class="flex h-full justify-evenly flex-wrap">
                    <Stat fig="28" label="new users this month"/>
                    <Stat fig="28" label="new users this month"/>
                    <Stat fig="28" label="new users this month"/>
                </div>
            </Panel>
        </div>
    }
}

#[component]
fn Stat(#[prop(into)] fig: TextProp, #[prop(into)] label: TextProp) -> impl IntoView {
    view! {
        <div class="basis-28 shrink">
            <h1 class="text-xl">{fig}</h1>
            <p>{label}</p>
        </div>
    }
}

#[component]
fn Activity() -> impl IntoView {
    view! {
        <div class="basis-2/3 grow max-w-prose">
            <Panel title="Activity">
                <Todo class="h-64 w-12"/>
            </Panel>
        </div>
    }
}

#[component]
fn RecentThreads() -> impl IntoView {
    let test_action = create_server_action::<GetDatabaseTest>();
    let (count, set_count) = create_signal(0);
    let _submitted = test_action.input();
    let pending = test_action.pending();
    let test_data = test_action.value();
    let unwrapped = move || {
        if let Some(x) = test_data.get() {
            match x {
                Ok(x) => x,
                _ => vec![],
            }
        } else {
            vec![]
        }
    };
    view! {
        <div class="grow basis-full">
            <Panel title="Recent Threads">
                <ActionForm action=test_action>
                    <button type="submit">"I'm basically a button"</button>
                    <div
                        class="cursor-pointer"
                        on:click=move |_| { set_count.set(count.get() + 1) }
                    >
                        {count}
                    </div>
                    <p>

                        {
                            move || pending().then(|| "Test");
                        }

                    </p>
                    <For each=unwrapped key=|state| state.id let:child>
                        <p>{child.name}</p>
                    </For>
                    <p>
                        {move || match test_data() {
                            Some(test) => test.unwrap().first().unwrap().clone().name,
                            None => "none".to_string(),
                        }}

                    </p>
                </ActionForm>
                <Todo class="h-64"/>
            </Panel>
        </div>
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Test {
    id: i32,
    name: String,
    created_at: String,
}
#[server(GetDatabaseTest)]
pub async fn get_database_test() -> Result<String, ServerFnError> {
    println!("testing");
    Ok("I'm a button")
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Panel title="404">
            <div class="p-4">"Not Found"</div>
        </Panel>
    }
}
