use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;

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
      <section
        class="hero min-h-screen bg-center bg-no-repeat bg-cover"
        aria-label="Introductory section"
      >
        <div class="hero-overlay" aria-hidden="true"></div>
        <div class="hero-content text-neutral-content text-center">
          <header class="max-w-md mx-auto">
            <h1 class="mb-5 text-5xl font-bold">Moin moin!</h1>
            <p class="mb-5">
              "Warum moin moin fragst du dich?
              Wenn es um Film geht, werde ich gesprächig. Deswegen schreib' mich gerne jederzeit an, wenn du Lust hast über Filmrollen zu schnacken!"
            </p>
            <a class="btn btn-primary" href="/contact" aria-label="Kontakt aufnehmen">
              Kontakt
            </a>
          </header>
        </div>
      </section>
    }
}

