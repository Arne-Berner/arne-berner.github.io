use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;

/// Simple contact page with Instagram and Email buttons.
/// - Replace the href values with your actual Instagram URL and email.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <NavBar/>
        <main class="min-h-screen flex items-center justify-center bg-base-200 p-6">
            <div class="card w-full max-w-md bg-base-100 shadow-xl">
                <div class="card-body items-center text-center">
                    <h1 class="card-title text-3xl">"Schnacken?"</h1>
                    <p class="text-base-content/70">"Schreib mir gerne einen Brief"</p>

                    <div class="divider"></div>

                    <div class="w-full space-y-3">
                        <a
                            href="mailto:info@arneberner.de"
                            class="btn btn-secondary w-full"
                        >
                            "Email"
                        </a>
                    </div>
                </div>
            </div>
        </main>
        <SiteFooter/>
    }
}
