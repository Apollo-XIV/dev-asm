use crate::components::panel::Panel as PanelTemplate;
use crate::models::comment;
use crate::utils::time_since;
use leptos::*;

#[component]
pub fn Panel(data: comment::Comment) -> impl IntoView {
    view! {
        <div>
            <PanelTemplate
                title={data.author}
                caption={format!("{} | {} ago",
                    data.date.format("%d/%m/%y"),
                    time_since(data.date))
            }>
                <p>{data.message}</p>
            </PanelTemplate>
        </div>
    }
}
