use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
// use leptos_router::components::A;
// TODO: make every picture into a component that just takes the image source file for clarity

/// Renders the home page of your application.
#[component]
pub fn Photos() -> impl IntoView {
    view! {
      <NavBar />
      <Carousel />
      <SiteFooter />
    }
}

#[component]
pub fn Carousel() -> impl IntoView {
    view! {
      <div class="carousel w-full">
        <div id="slide1" class="carousel-item relative max-h-screen">
          <img
            class="max-h-screen object-cover"
            src="./photos/DSC01699-800.webp"
            srcset="
            ./photos/DSC01699-400.webp 400w,
            ./photos/DSC01699-600.webp 600w,
            ./photos/DSC01699-800.webp 800w,
            ./photos/DSC01699-1024.webp 1024w,
            ./photos/DSC01699-1200.webp 1200w,
            ./photos/DSC01699-1600.webp 1600w,
            ./photos/DSC01699-1920.webp 1920w
            "
            loading="eager"
            alt="Ein Portraitbild welches den Schauspieler Arne Berner zeigt,
            der in die Kamera schaut."
            decoding="async"
            fetchpriority="high"
          />
          <div class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">
            <a href="#slide4" class="btn btn-circle">
              "❮"
            </a>
            <a href="#slide2" class="btn btn-circle">
              "❯"
            </a>
          </div>
        </div>
        <div id="slide2" class="carousel-item relative max-h-screen">
          <img
            class="max-h-screen object-cover"
            src="./photos/DSC01810-800.webp"
            srcset="
            ./photos/DSC01810-400.webp 400w,
            ./photos/DSC01810-600.webp 600w,
            ./photos/DSC01810-800.webp 800w,
            ./photos/DSC01810-1024.webp 1024w,
            ./photos/DSC01810-1200.webp 1200w,
            ./photos/DSC01810-1600.webp 1600w,
            ./photos/DSC01810-1920.webp 1920w
            "
            loading="eager"
            alt="Ein Portraitbild welches den Schauspieler Arne Berner zeigt,
            der in die Kamera schaut."
            decoding="async"
            fetchpriority="high"
          />
          <div class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">
            <a href="#slide1" class="btn btn-circle">
              "❮"
            </a>
            <a href="#slide3" class="btn btn-circle">
              "❯"
            </a>
          </div>
        </div>
        <div id="slide3" class="carousel-item relative w-full">
          <img
            src="https://img.daisyui.com/images/stock/photo-1414694762283-acccc27bca85.webp"
            class="max-h-screen object-cover"
          />
          <div class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">
            <a href="#slide2" class="btn btn-circle">
              "❮"
            </a>
            <a href="#slide4" class="btn btn-circle">
              "❯"
            </a>
          </div>
        </div>
        <div id="slide4" class="carousel-item relative w-full">
          <img
            src="https://img.daisyui.com/images/stock/photo-1665553365602-b2fb8e5d1707.webp"
            class="max-h-screen object-cover"
          />
          <div 
          />
          <div class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between">
            <a href="#slide3" class="btn btn-circle">
              "❮"
            </a>
            <a href="#slide1" class="btn btn-circle">
              "❯"
            </a>
          </div>
        </div>
      </div>
    }
}

