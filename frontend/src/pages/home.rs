// frontend/src/pages/home.rs

use crate::app::Route;
use crate::components::book_card::BookCard;
use crate::components::event_card::EventCard;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <section class="hero">
                <h1>{"Element "}<span class="hero-accent">{"Community"}</span></h1>
                <p class="hero-subtitle">{"Books, runs, and everything in between"}</p>
            </section>

            // --- Upcoming Events (sorted by date) ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// upcoming"}</span>
                </div>

                // Run Club — April 18
                <EventCard
                    title="Saturday Morning Run"
                    date="Saturday, April 18"
                    description="Join us for this week's group run. \"All paces welcome, we start together and finish together!\""
                    icon="directions_run"
                    image_url="https://d3o5xota0a1fcr.cloudfront.net/v6/maps/2P3KS7NLCSFNKIATCIYWJV64CCQVKCLZEPQNWVXGRAXYOW5ARUIAVR6N5FOCLFS3CZVUCH4ZF64GPT3WTACH4WZO22RXVJFPGDY3SZBTFZK6WKQ6RCNLTW6VFZGKPZCHEU7QP5L4YRFJHBDYN2CY2FVH5M5JGJRALYYCI3SO6FM2HINS7NGSAMJ45ZPMWEC674======"
                    badge_label="Run Club"
                    badge_icon="directions_run"
                    secondary_label="View Route"
                    secondary_url="https://www.strava.com/routes/3480270817809452530"
                />

                // Community Events — April 18
                <EventCard
                    title="HYROX PFT | April 18th"
                    date="Saturday, April 18 · 11:00 AM EDT"
                    location="667 Grand St, Brooklyn, NY 11211"
                    description="The official HYROX Physical Fitness Test — 6 stations for time: 1000m run, 50 burpee broad jumps, 100 stationary lunges, 1000m row, 30 hand-release push-ups, 100 wall balls. Running heats at 11am and 12pm. Open to all levels."
                    icon="celebration"
                    image_url="https://withforte.co/_next/image?url=https%3A%2F%2Ffleetath.s3.us-east-1.amazonaws.com%2Ffile_20260324202010420_29119816.PNG&w=1200&q=75"
                    badge_label="Community"
                    badge_icon="celebration"
                    rsvp_url="https://withforte.co/events/wdsbGbuxMw6ynjLMrcHj"
                />

                // Book Club — April 2026 (ongoing)
                <BookCard
                    title="Headshot"
                    author="Rita Bullwinkel"
                    description="An electrifying novel about a women's boxing tournament in Reno, Nevada, featuring eight teenage competitors. The narrative mirrors the tournament bracket, weaving together each fighter's past, present, and future while exploring themes of ambition, physicality, and the fleeting nature of pivotal moments."
                    cover_url="https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg"
                    month="April 30, 2026"
                    goodreads_url="https://www.goodreads.com/book/show/174156218"
                    partiful_url="https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"
                    is_current=true
                    featured=true
                />
            </section>

            // --- Hub Cards ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// explore"}</span>
                </div>
                <div class="info-grid">
                    <Link<Route> to={Route::BookClub} classes="info-card home-hub-card">
                        <span class="info-icon material-symbols-outlined">{"auto_stories"}</span>
                        <h3>{"Book Club"}</h3>
                        <p>{"One book a month. Read it, meet up, talk about it."}</p>
                        <span class="home-hub-link">{"See what we're reading →"}</span>
                    </Link<Route>>
                    <Link<Route> to={Route::RunClub} classes="info-card home-hub-card">
                        <span class="info-icon material-symbols-outlined">{"directions_run"}</span>
                        <h3>{"Run Club"}</h3>
                        <p>{"Group runs every Wednesday and Saturday. All paces welcome."}</p>
                        <span class="home-hub-link">{"See the schedule →"}</span>
                    </Link<Route>>
                    <Link<Route> to={Route::Events} classes="info-card home-hub-card">
                        <span class="info-icon material-symbols-outlined">{"celebration"}</span>
                        <h3>{"Community Events"}</h3>
                        <p>{"Socials, fitness events, and one-offs for the whole crew."}</p>
                        <span class="home-hub-link">{"See upcoming events →"}</span>
                    </Link<Route>>
                </div>
            </section>
        </>
    }
}
