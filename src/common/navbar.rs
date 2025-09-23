use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
      <div class="navbar bg-base-100 shadow-sm">
        <div class="navbar-start">
          <a class="btn btn-ghost text-xl" href="/">
            "Arne Berner"
          </a>
        </div>
        <div class="navbar-center hidden lg:flex">
          <ul class="menu menu-horizontal px-1">
            <li>
              <a href="/vita">Vita</a>
            </li>
            <li>
              <a href="/photos">Photos</a>
            </li>
            <li>
              <a href="/showreel">Showreel</a>
            </li>
            <li>
              <a href="/audio">Audio</a>
            </li>
            <li>
              <a href="/contact">Kontakt</a>
            </li>
          </ul>
        </div>
        <div class="navbar-end">
          <div class="dropdown dropdown-end">
            <div tabindex="0" role="button" class="btn btn-ghost lg:hidden">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="24"
                height="24"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <rect x="3" y="6" width="18" height="1" rx="1" />
                <rect x="3" y="11" width="18" height="1" rx="1" />
                <rect x="3" y="16" width="18" height="1" rx="1" />
              </svg>
            </div>
            <ul
              tabindex="0"
              class="menu menu-sm dropdown-content bg-base-100 rounded-box z-1 mt-3 w-52 p-2 shadow"
            >
              <li>
                <a href="/vita">Vita</a>
              </li>
              <li>
                <a href="/photos">Photos</a>
              </li>
              <li>
                <a href="/showreel">Showreel</a>
              </li>
              <li>
                <a href="/audio">Audio</a>
              </li>
              <li>
                <a href="/contact">Kontakt</a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    }
}
