use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::{panel::Panel, nav::Nav, todo::Todo};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-start.css"/>
        <Stylesheet id="leptos" href="/assets/tailwind.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <body class="bg-slate-950 flex place-items-center min-h-screen">
                <Nav />
                <main class="max-w-3xl h-fit flex flex-wrap justify-center place-items-stretch gap-2 w-full p-2">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="/*any" view=NotFound/>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Stats />
        <Activity />
        <RecentThreads />
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
fn Stat(
    #[prop(into)]
    fig: TextProp,
    #[prop(into)]
    label: TextProp
) -> impl IntoView {
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
    view! {
        <div class="grow basis-full">
            <Panel title="Recent Threads">
                <Todo class="h-64"/>
            </Panel>
        </div>
    }
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
        <Panel title="404"><div class="p-4">"Not Found"</div></Panel>
    }
}
