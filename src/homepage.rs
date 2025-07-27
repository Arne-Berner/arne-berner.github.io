use leptos::prelude::*;
use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <h1>"Bald ist diese Seite wieder Verfügbar!"</h1>
        <img
            src="/construction-800.webp"
            srcset="
                /construction-400.webp 400w,
                /construction-600.webp 600w,
                /construction-800.webp 800w,
                /construction-1024.webp 1024w,
                /construction-1200.webp 1200w,
                /construction-1600.webp 1600w,
                /construction-1920.webp 1920w
            "
            sizes="(max-width: 600px) 100vw, (max-width: 1200px) 80vw, 800px"
            alt="My Photo"
            style="width: 100%; height: auto; border-radius: 8px;"
        />
        <Counter/>
        <A href="/sub-page">"Sub page"</A>
        <A href="/another-page">"Another page"</A> /
    }
}

#[island]
fn Counter() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <button on:click=on_click>"Click Me: " {count}</button>
    }

}
