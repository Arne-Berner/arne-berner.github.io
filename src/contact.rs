use leptos::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;

/// Simple contact page with Instagram and Email buttons.
/// - Replace the href values with your actual Instagram URL and email.
#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Navbar/>
        <main class="min-h-screen flex items-center justify-center bg-base-200 p-6">
            <div class="card w-full max-w-md bg-base-100 shadow-xl">
                <div class="card-body items-center text-center">
                    <h1 class="card-title text-3xl">"Contact Me"</h1>
                    <p class="text-base-content/70">"I’d love to hear from you!"</p>

                    <div class="divider"></div>

                    <div class="w-full space-y-3">
                        <a
                            href="https://instagram.com/your_instagram_username"
                            target="_blank"
                            rel="noopener noreferrer"
                            class="btn btn-outline btn-primary w-full"
                        >
                            "Instagram"
                        </a>

                        <a
                            href="mailto:your@email.com"
                            class="btn btn-secondary w-full"
                        >
                            "Email Me"
                        </a>
                    </div>
                </div>
            </div>
        </main>
        <SiteFooter/>
    }
}
