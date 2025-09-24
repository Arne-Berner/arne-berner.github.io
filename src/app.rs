use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path,
    static_routes::StaticRoute,
    SsrMode,
};
use crate::homepage::HomePage;
use crate::vita::Vita;
use crate::photos::Photos;
use crate::audio::Audio;
use crate::video::Video;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="de" data-theme="cupcake">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <meta
            name="description"
            content="Author: Arne Berner,
            Vita, Showreel, Fotos und Kontakt für den Schauspieler Arne Berner,
            Single Page Application"
          />
          <AutoReload options=options.clone() />
          <HydrationScripts options islands=true />
          <MetaTags />
        </head>
        <body>
          <App />
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
      <Stylesheet id="leptos" href="/pkg/arneberner.css" />

      // sets the document title
      <Title text="Arne Berner" />

      // content for this welcome page
      <Router>
        <main>
          <FlatRoutes fallback=|| "Page not found.".into_view()>
            <Route path=path!("/") view=HomePage ssr=SsrMode::Static(StaticRoute::new()) />
            <Route path=path!("/vita") view=Vita ssr=SsrMode::Static(StaticRoute::new()) />
            <Route path=path!("/photos") view=Photos ssr=SsrMode::Static(StaticRoute::new()) />
            <Route path=path!("/audio") view=Audio ssr=SsrMode::Static(StaticRoute::new()) />
           <Route path=path!("/video") view=Video ssr=SsrMode::Static(StaticRoute::new()) />
          </FlatRoutes>
        </main>
      </Router>
    }
}

