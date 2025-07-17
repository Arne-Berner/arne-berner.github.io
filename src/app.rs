use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use crate::subpage::SubPage;
use crate::anotherpage::AnotherPage;
use leptos_router::components::A;
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ssg-test.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <FlatRoutes fallback=|| "Page not found.".into_view()>
                    <Route
                        path=path!("/")
                        view=HomePage
                        ssr=SsrMode::Static(StaticRoute::new())
                    /> 
                    <Route
                        path=path!("/sub-page")
                        view=SubPage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                    <Route
                        path=path!("/another-page")
                        view=AnotherPage
                        ssr=SsrMode::Static(StaticRoute::new())
                    />
                </FlatRoutes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <h1>"Welcome to Leptos!"</h1>
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
