use leptos::prelude::*;
// use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let actor_name = "Arne Berner";
    let copyright_owner = "Hannes Gorissen";
    let copyright_year = "2025";
    let copyright_by = format!("© {copyright_owner} {copyright_year}");


    view! {
      <nav class="nav">
        <NavBar/>
      </nav>
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
    }
}

#[island]
fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);

    let toggle = move |_| set_open.update(|o| *o = !*o);
    let close = move |_| set_open.set(false);

    view! {
        <div class="nav-inner">
            <div class="name">Arne Berner</div>
            <button
                class="burger"
                aria-label="Toggle menu"
                aria-controls="nav-menu"
                aria-expanded=move || open.get().to_string()
                on:click=toggle
            >
                <span></span>
                <span></span>
                <span></span>
            </button>
            <div
                id="nav-menu"
                class="menu"
                class:open=move || open.get()
                on:click=close
            >
                <a href="/vita">Vita</a>
                <a href="/photos">Photos</a>
                <a href="/showreel">Showreel</a>
            </div>
        </div>
    }

}
