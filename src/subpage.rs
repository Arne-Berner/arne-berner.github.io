use leptos::prelude::*;

#[component]
pub fn SubPage() -> impl IntoView {
    view! {
        <h1>"SubPage"</h1>
        <Sub_Counter/>
    }
}

#[island]
fn Sub_Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <p>"This is a subpage with interactivity: " {count}</p>
        <button on:click=on_click>"Increment"</button>
    }

}


