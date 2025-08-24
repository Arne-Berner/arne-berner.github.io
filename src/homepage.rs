use leptos::prelude::*;
use crate::common::{NavBar, SiteFooter};
// use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let copyright_owner = "Hannes Gorissen";
    let copyright_year = "2025";
    let copyright_by = format!("© {copyright_owner} {copyright_year}");


    view! {
      <NavBar/>
      <main class="viewport">
      <figure class="hero" role="group" aria-label="Arne Berner Portrait">
        <img
            src="/Headshot-800.webp"
            srcset="
              /Headshot-400.webp 400w,
              /Headshot-600.webp 600w,
              /Headshot-800.webp 800w,
              /Headshot-1024.webp 1024w,
              /Headshot-1200.webp 1200w,
              /Headshot-1600.webp 1600w,
              /Headshot-1920.webp 1920w
            "
          loading="eager"
          alt="Ein Portraitbild welches den Schauspieler Arne Berner zeigt,
            der in die Kamera schaut."
          decoding="async"
          fetchpriority="high"
        />
          <figcaption class="copyright">{copyright_by}</figcaption>
        </figure>
      </main>
      <SiteFooter/>
    }
}

