use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
use crate::common::todo::TODO;

/// Renders the Audiopage for your website
#[component]
pub fn Showreel() -> impl IntoView {
    view! {
      <NavBar />
      <TODO />
      <SiteFooter />
    }
}
