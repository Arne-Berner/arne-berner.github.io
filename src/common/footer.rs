use leptos::prelude::*;

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
      <footer class="site-footer">
        <div class="footer-inner">
          <nav aria-label="Legal links" class="legal-nav">
            <a href="/impressum" class="impressum-link">
              "Impressum"
            </a>
          </nav>
          <p class="site-copy">"©ArneBerner 2025"</p>
        </div>
      </footer>
    }
}
