use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
      <nav class="navbar bg-base-100 shadow-sm" aria-label="Main navigation">
        <div class="navbar-start">
          <a class="btn btn-ghost text-5xl" href="/" aria-label="Home page: Arne Berner">
            Arne Berner
          </a>
        </div>
        <div class="navbar-center hidden lg:flex">
          <ul class="menu menu-horizontal px-1" role="menubar" aria-label="Main menu">
            <li>
              <a href="/vita" role="menuitem">
                Vita
              </a>
            </li>
            <li>
              <a href="/photos" role="menuitem">
                Photos
              </a>
            </li>
            <li>
              <a href="/showreel" role="menuitem">
                Showreel
              </a>
            </li>
            <li>
              <a href="/audio" role="menuitem">
                Audio
              </a>
            </li>
            <li>
              <a href="/contact" role="menuitem">
                Kontakt
              </a>
            </li>
          </ul>
        </div>
        <div class="navbar-end">
          <div class="dropdown dropdown-end">
            <button
              tabindex="0"
              role="button"
              class="btn btn-ghost lg:hidden"
              aria-label="Open main menu"
              aria-controls="mobile-menu"
              aria-expanded="false"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                aria-hidden="true"
                focusable="false"
              >
                <rect x="3" y="6" width="18" height="1" rx="1" />
                <rect x="3" y="11" width="18" height="1" rx="1" />
                <rect x="3" y="16" width="18" height="1" rx="1" />
              </svg>
            </button>
            <ul
              id="mobile-menu"
              tabindex="0"
              class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
              role="menu"
              aria-label="Mobile menu"
            >
              <li>
                <a href="/vita" role="menuitem">
                  Vita
                </a>
              </li>
              <li>
                <a href="/photos" role="menuitem">
                  Photos
                </a>
              </li>
              <li>
                <a href="/showreel" role="menuitem">
                  Showreel
                </a>
              </li>
              <li>
                <a href="/audio" role="menuitem">
                  Audio
                </a>
              </li>
              <li>
                <a href="/contact" role="menuitem">
                  Kontakt
                </a>
              </li>
            </ul>
          </div>
        </div>
      </nav>
    }
}
