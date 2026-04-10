// frontend/src/pages/schedule.rs
//
// Reading schedule page with a timeline and card for each month's pick.

use crate::components::book_card::BookCard;
use yew::prelude::*;

#[function_component(Schedule)]
pub fn schedule() -> Html {
    html! {
        <div class="schedule-page">
            <h1 class="schedule-page-title">{"Reading Schedule"}</h1>
            <p class="schedule-page-subtitle">
                {"One book a month. Here's what we're reading and what's coming up."}
            </p>

            // --- Timeline ---
            <div class="timeline">
                <div class="timeline-item active">
                    <div class="timeline-month">{"April 2026"}</div>
                    <div class="timeline-book">{"Headshot"}</div>
                    <div class="timeline-author">{"Rita Bullwinkel"}</div>
                </div>
                <div class="timeline-item">
                    <div class="timeline-month">{"May 2026"}</div>
                    <div class="timeline-book">{"TBD"}</div>
                    <div class="timeline-author">{
                        "Suggest one → "
                    }
                    <a href="https://forms.gle/eQFo1SqXJwRfr3tX6" target="_blank" rel="noopener">
                        {"submit a pick"}
                    </a>
                    </div>
                </div>
            </div>

            // --- Current pick detail card ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// april 2026"}</span>
                </div>

                <BookCard
                    title="Headshot"
                    author="Rita Bullwinkel"
                    description="An electrifying novel about a women's boxing tournament in Reno, Nevada, featuring eight teenage competitors. Each fighter arrives with distinct motivations shaped by personal tragedy, family pressure, and the desire for control. The prose employs sharp, rapid-fire sentences that mirror boxing's intensity. Longlisted for the Booker Prize and a Pulitzer finalist."
                    cover_url="https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg"
                    month="April 2026"
                    goodreads_url="https://www.goodreads.com/book/show/174156218"
                    partiful_url="https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"
                    is_current=true
                    featured=true
                />
            </section>
        </div>
    }
}
