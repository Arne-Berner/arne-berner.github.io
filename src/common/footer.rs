use leptos::prelude::*;

#[component]
pub fn SiteFooter() -> impl IntoView {
    let copyright_owner = "Hannes Gorissen";
    let copyright_year = "2025";
    let copyright_by = format!("© {copyright_owner} {copyright_year}");
    view! {
        <footer class="footer footer-horizontal footer-center bg-base-200 text-base-content rounded p-10" aria-label="Footer">
  <nav class="flex flex-wrap justify-center gap-4" role="navigation" aria-label="Footer navigation">
    <ul class="flex flex-wrap justify-center gap-4 m-0 p-0 list-none">
      <li>
        <a class="link link-hover" href="/vita" aria-label="Vita">Vita</a>
      </li>
      <li>
        <a class="link link-hover" href="/photos" aria-label="Photos">Photos</a>
      </li>
      <li>
        <a class="link link-hover" href="/showreel" aria-label="Showreel">Showreel</a>
      </li>
      <li>
        <a class="link link-hover" href="/audio" aria-label="Audio">Audio</a>
      </li>
      <li>
        <a class="link link-hover" href="/contact" aria-label="Kontakt">Kontakt</a>
      </li>
      <li>
        <a class="link link-hover" href="/impressum" aria-label="Impressum">Impressum</a>
      </li>
    </ul>
  </nav>
  <aside>
    <p>
      "Photo Copyright © "{copyright_by}
    </p>
  </aside>
</footer>
    }
}
