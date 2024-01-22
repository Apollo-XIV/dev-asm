use leptos::*;

#[component]
pub fn Panel(
	#[prop(optional, into)]
	class: TextProp,
	#[prop(optional, into)]
	title: String,
	children: Children 
) -> impl IntoView {
	view! {
		<div class={format!("p-1 bg-amber-300 text-white rounded-sm flex flex-col {}", class.get())}>
			<h1 class="pl-2 text-lg text-blue-950 font-bold italic">{title}</h1>
			<div class="p-2 bg-blue-950 rounded-sm grow">
			 	{children()}
			</div>
		</div>
	}
}
