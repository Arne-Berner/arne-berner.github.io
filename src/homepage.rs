use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
// use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let copyright_owner = "Hannes Gorissen";
    let copyright_year = "2025";
    let copyright_by = format!("© {copyright_owner} {copyright_year}");


    view! {
      <NavBar />
      <Hero />
      <SiteFooter />
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    view! {
      <div class="hero min-h-screen" style="background-image: url('/Headshot-1920.webp');">
        <div class="hero-overlay"></div>
        <div class="hero-content text-neutral-content text-center">
          <div class="max-w-md">
            <h1 class="mb-5 text-5xl font-bold">"Moin moin!"</h1>
            <p class="mb-5">"Warum moin moin? Wenn es um Film geht, werde ich gesprächig."</p>
            <a class="btn btn-primary" href="/contact">
              "Kontakt"
            </a>
          </div>
        </div>
      </div>
    }
}

