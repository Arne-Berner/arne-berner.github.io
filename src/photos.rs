use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
// TODO: Add Gallery view Button (pop up window)

/// Renders the home page of your application.
#[component]
pub fn Photos() -> impl IntoView {
    view! {
      <NavBar />
      <Carousel />
      <SiteFooter />
    }
}

pub struct SlideData{
    filename: String,
    number: usize,
}

impl SlideData {
    pub fn new(filename: String, number: usize) -> Self {
        SlideData { filename, number }
    }

}

/// A Carousel that uses daisy uis center carousel with space between each card
#[component]
pub fn Carousel() -> impl IntoView {
    let file_idents = ["2049", "1777", "1810", "2049", "1832"];
    let filenames = file_idents.iter()
        .enumerate()
        .map(|(num, ident)| {
            let filename = format!("DSC0{}",ident);
            let slide_data = SlideData::new(filename, num);
            view!{<Slide slide_data={slide_data}/>}
        })
        .collect::<Vec<_>>();
        
    view! {
      <div class="carousel carousel-center w-full bg-neutral space-x-4 p-4">
        {filenames}
      </div>
    }
}

#[component]
pub fn Slide(
    slide_data: SlideData,
     #[prop(optional)]
     alt: Option<String>
     ) -> impl IntoView {
    let resolutions = [400, 600, 800, 1024, 1200, 1600, 1920];
    let srcset = resolutions.iter()
        .map(|w| format!("./photos/{}-{}.webp {}w", slide_data.filename, w, w))
        .collect::<Vec<_>>()
        .join(", ");
    let default_src = format!("./photos/{}-800.webp", slide_data.filename);
    let alt_text = alt.unwrap_or_else(|| slide_data.filename.clone());
    let current = format!("slide{}", slide_data.number);
    view! { 
        <div id=current class="carousel-item relative max-h-screen">
            <img
                class="max-h-screen object-cover"
                src=default_src
                srcset=srcset
                loading="eager"
                alt=alt_text
                decoding="async"
                fetchpriority="high"
            />
        </div>
    }
}
