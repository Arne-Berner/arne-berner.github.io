use leptos::prelude::*;
// use leptos_router::components::A;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <nav class="nav">
            <NavBar/>
        </nav>
        <h1>"Photos"</h1>
        <div class="photos-list">
          <img
              src="/DSC01871-800.webp"
              srcset="
                  /DSC01871-400.webp 400w,
                  /DSC01871-600.webp 600w,
                  /DSC01871-800.webp 800w,
                  /DSC01871-1024.webp 1024w,
                  /DSC01871-1200.webp 1200w,
                  /DSC01871-1600.webp 1600w,
                  /DSC01871-1920.webp 1920w
              "
              sizes="(max-width: 600px) 100vw, 800px"
              alt="My Photo"
              style="width: 100%; height: auto; border-radius: 8px;"
          />
        </div>
    }
}

#[island]
fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);

    let toggle = move |_| set_open.update(|o| *o = !*o);
    let close = move |_| set_open.set(false);

    view! {
        <div class="nav-inner">
            <div class="name">Arne</div>
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
