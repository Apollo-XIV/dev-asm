use crate::components::panel::Panel;
use crate::models::thread;
use leptos::*;
use crate::utils;

#[component]
pub fn Thread(#[prop(into)] data: thread::Thread) -> impl IntoView {
    view! {
        <div class="grow w-full max-w-3xl">
            <Panel 
                title={data.author} 
                caption={format!("{} | {} ago",data.date.format("%d/%m/%y"), utils::time_since(data.date))}
            >
                <a href={format!("/forum/{}", data.id)} class="text-lg font-bold hover:underline">
                    {data.title}
                </a>
            </Panel> 
        </div>
    }
}

#[component]
pub fn Header(data: thread::Thread) -> impl IntoView {
    view! {
        <div class="p-2 bg-amber-300 text-blue-950 rounded-sm w-full flex flex-col">
            <h1 class="text-lg font-bold">{data.title}</h1>
            <div class="flex justify-between">
                <p>{data.author}</p>
                <p>{format!("{}", data.date.format("%d/%m/%y"))}</p>
            </div>
        </div>
    }
}
