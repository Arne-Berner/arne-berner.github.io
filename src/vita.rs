use leptos::prelude::*;
use crate::common::navbar::NavBar;
use crate::common::footer::SiteFooter;
// TODO: 
// - make each timeline point into a component
// - create each timeline in vita
// - zip each timeline with the fitting topic 

#[component]
pub fn Vita() -> impl IntoView {
    // Topics and their respective CV details



    view! {
      <NavBar />
      <Tabs />
      <SiteFooter />
    }
}


#[component]
fn Tabs() -> impl IntoView {
    view! {
      <div role="tablist" class="tabs tabs-border justify-center">
        <input
          type="radio"
          name="my_tabs_3"
          class="tab text-xl"
          aria-label="Film & Fernsehen"
          checked="checked"
        />
        <div class="tab-content bg-base-100 border-base-300 p-6">
          <FilmTimeline />
        </div>
        <input type="radio" name="my_tabs_3" class="tab text-xl" aria-label="Theater" />
        <div class="tab-content bg-base-100 border-base-300 p-6">
          <TheaterTimeline />
        </div>
        <input type="radio" name="my_tabs_3" class="tab text-xl" aria-label="Werbung & Diverses" />
        <div class="tab-content bg-base-100 border-base-300 p-6">
          <DiversTimeline />
        </div>
      </div>
    }
}

#[component]
fn FilmTimeline() -> impl IntoView {
    view! {
      <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
        <li>
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2024</time>
            <div class="text-lg font-black">"Idol"</div>
"Kurzfilm
Friedrich - Hauptrolle
Regie: Johann Schultz & Merlin Slamanig
Produktion: Johann Schultz Produktion & Moinmoin Movies
Casting: - "
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-end whitespace-pre-wrap md:mb-10">
            <time class="font-mono italic">2023</time>
            <div class="text-lg font-black">"Die Flut - Tod am Deich"</div>
"Spielfilm
Zivi Mark - Nebenrolle
Regie: Andreas Prochaska
Produktion: ARD
Casting: Deborah Congia"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2023</time>
            <div class="text-lg font-black">"Ragnhild"</div>
"Kurzfilm
Korbinian - Hauptrolle
Regie: Roland Cremerius
Produktion: HFBK
Casting: -"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-end whitespace-pre-wrap md:mb-10">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Aufwärts"</div>
"Kurzfilm
Liftboy - Hauptrolle
Regie: Johann Schultz
Produktion: Johann Schultz Produktion
Casting: -"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Die Richterin"</div>
"Kurzfilm
Tom - Nebenrolle
Regie: Tim Boye
Produktion: Moinmoin Movies
Casting: -"
          </div>
        </li>
      </ul>
    }
}

#[component]
fn TheaterTimeline() -> impl IntoView {
    view! {
      <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
        <li>
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2020</time>
            <div class="text-lg font-black">"Pipi plündert den Weihnachtsbaum"</div>
"Kinderstück
Tommy - Nebenrolle
Regie: Andreas Kloos
Theater: Landesbühne Niedersachsen Nord"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-end whitespace-pre-wrap md:mb-10">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Feuerzangenbowle"</div>
"Boulevardtheater
Ackermann - Nebenrolle
Regie: Christian Voss
Theater: Komödie am Altstadtmarkt"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Kleider machen Leute"</div>
"Jugendtheater
Manager, Wirt, Melchior Böhmi - Nebenrolle
Regie: Liv Manthey
Theater: Kreuzgangspiele"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-end md:whitespace-pre-wrap mb-10">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Der Liebhaber"</div>
"Kammerspiel
Milchmann -  Nebenrolle
Regie: Daniel Karasek
Theater: Theater Kiel"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2019</time>
            <div class="text-lg font-black">"Messer in Hennen"</div>
"Kammerspiel
Gilbert Horn - Hauptrolle
Regie: Mathisek Brokhues
Theater: Schule für Schauspiel Kiel"
          </div>
        </li>
      </ul>
    }
}

#[component]
fn DiversTimeline() -> impl IntoView {
    view! {
      <ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
        <li>
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2024</time>
            <div class="text-lg font-black">"ADHS-App"</div>
"Videos für eine App per Prompter eingelese
Person mit ADHS
Regie: Jörg Daniel Hissen
Produktion: Riversidefilm"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-end md:whitespace-pre-wrap mb-10">
            <time class="font-mono italic">2023</time>
            <div class="text-lg font-black">"Umgang mit Patienten"</div>
"Training für junge Ärzt*innen
In Zusammenarbeit mit der Augenklinik Bellevue"
          </div>
          <hr />
        </li>
        <li>
          <hr />
          <div class="timeline-middle">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              class="h-5 w-5"
            >
              <path
                fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z"
                clip-rule="evenodd"
              />
            </svg>
          </div>
          <div class="timeline-start whitespace-pre-wrap mb-10 md:text-end">
            <time class="font-mono italic">2022</time>
            <div class="text-lg font-black">"PACE S-Payment"</div>
"Werbung
Enkelsohn - Hauptrolle
Regie: Peter Ahlers
Produktion: Kabuja"
          </div>
          <hr />
        </li>
      </ul>
    }
}
