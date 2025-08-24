use leptos::prelude::*;

#[island]
pub fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);

    let toggle = move |_| set_open.update(|o| *o = !*o);
    let close = move |_| set_open.set(false);

    view! {
      <nav class="nav">
        <div class="nav-inner">
          <a class="name" href="/">Arne Berner</a>
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
            <a href="/audio">Vita</a>
            <a href="/contact">Photos</a>
          </div>
        </div>
      </nav>
    }
}

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="footer-inner">
                <nav aria-label="Legal links" class="legal-nav">
                    <a href="/impressum" class="impressum-link">"Impressum"</a>
                </nav>
                <p class="site-copy">
                    "©ArneBerner 2025"
                </p>
            </div>
        </footer>
    }
}
