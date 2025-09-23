use leptos::prelude::*;

#[component]
pub fn SiteFooter() -> impl IntoView {
    let copyright_owner = "Hannes Gorissen";
    let copyright_year = "2025";
    let copyright_by = format!("© {copyright_owner} {copyright_year}");
    view! {
      <footer class="footer footer-horizontal footer-center bg-base-200 text-base-content rounded p-10">
        <nav class="flex flex-wrap justify-center gap-4">
          <a class="link link-hover" href="/vita">
            Vita
          </a>
          <a class="link link-hover" href="/photos">
            Photos
          </a>
          <a class="link link-hover" href="/showreel">
            Showreel
          </a>
          <a class="link link-hover" href="/audio">
            Audio
          </a>
          <a class="link link-hover" href="/contact">
            Kontakt
          </a>
          <a class="link link-hover" href="/impressum">
            Impressum
          </a>
        </nav>
        <aside>
          <p>"Photo Copyright "{copyright_by}</p>
        </aside>
      </footer>
    }
}
