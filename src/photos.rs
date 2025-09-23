use leptos::prelude::*;
use crate::common::{NavBar, SiteFooter};

#[component]
pub fn Photos() -> impl IntoView {
    view! {
      <NavBar />
      <body>
        <main class="gallery" aria-label="Actor photo gallery">
          <div class="carousel">
            <div class="card">
              <h3>"Card 1"</h3>
            </div>
            <div class="card">
              <h3>"Card 2"</h3>
            </div>
            <div class="card">
              <h3>"Card 3"</h3>
            </div>
            <div class="card">
              <h3>"Card 4"</h3>
            </div>
            <div class="card">
              <h3>"Card 5"</h3>
            </div>
            <div class="card">
              <h3>"Card 6"</h3>
            </div>
            <div class="card">
              <h3>"Card 7"</h3>
            </div>
            <div class="card">
              <h3>"Card 8"</h3>
            </div>
            <div class="card">
              <h3>"Card 9"</h3>
            </div>
            <div class="card">
              <h3>"Card 10"</h3>
            </div>
            <div class="card">
              <h3>"Card 11"</h3>
            </div>
            <div class="card">
              <h3>"Card 12"</h3>
            </div>
          </div>
        </main>
      </body>
      <SiteFooter />
    }
}
