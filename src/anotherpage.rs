use leptos::prelude::*;

#[component]
pub fn AnotherPage() -> impl IntoView {
    view! {
        <h1>"Another Page"</h1>
        <WindowHeight/>
    }
}

#[island]
fn WindowHeight() -> impl IntoView {
    let (window_height, set_window_height) = signal::<Option<f64>>(None);
    let window_text = move || {
        window_height()
            .map(|w| format!(": {}", w))
            .unwrap_or("".to_owned())
    };

    // Ensure that the window object is available before calling browser APIs.
    Effect::new(move |_| {
        set_window_height(window().inner_height().ok().map(|w| w.as_f64().unwrap()));
    });
    view! {
        <p>"This is a another page calling browser APIs" {window_text}</p>
    }

}
