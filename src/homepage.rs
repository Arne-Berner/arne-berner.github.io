use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
// use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
      <NavBar />
      <Hero />
      <SiteFooter />
    }
}

#[component]
pub fn Hero() -> impl IntoView {
    view! {
      <div
        class="hero min-h-screen bg-center bg-no-repeat bg-cover"
      >
        <div class="hero-overlay"></div>
        <div class="hero-content text-neutral-content text-center">
          <div class="max-w-md">
            <h1 class="mb-5 text-5xl font-bold">"Moin moin!"</h1>
            <p class="mb-5">"Warum moin moin fragst du dich? 
            Wenn es um Film geht, werde ich gesprächig. Deswegen schreib' mich gerne jederzeit an, wenn du Lust hast über Filmrollen zu schnacken!"</p>
            <a class="btn btn-primary" href="/contact">
              "Kontakt"
            </a>
          </div>
        </div>
      </div>
    }
}

