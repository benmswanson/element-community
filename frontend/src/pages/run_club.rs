// frontend/src/pages/run_club.rs

use crate::components::event_card::EventCard;
use yew::prelude::*;

#[function_component(RunClub)]
pub fn run_club() -> Html {
    html! {
        <>
            // --- Hero ---
            <section class="hero">
                <h1>{"Element "}<span class="hero-accent">{"Run Club"}</span></h1>
                <p class="hero-subtitle">
                    {"Get moving with the Element community"}
                </p>
            </section>

            // --- Next Run ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// next run"}</span>
                </div>
                <EventCard
                    title="Saturday Morning Run"
                    date="Saturday, April 18"
                    description="Join us for this week's group run. \"All paces welcome, we start together and finish together!\""
                    icon="directions_run"
                    image_url="https://d3o5xota0a1fcr.cloudfront.net/v6/maps/2P3KS7NLCSFNKIATCIYWJV64CCQVKCLZEPQNWVXGRAXYOW5ARUIAVR6N5FOCLFS3CZVUCH4ZF64GPT3WTACH4WZO22RXVJFPGDY3SZBTFZK6WKQ6RCNLTW6VFZGKPZCHEU7QP5L4YRFJHBDYN2CY2FVH5M5JGJRALYYCI3SO6FM2HINS7NGSAMJ45ZPMWEC674======"
                    badge_label="Upcoming"
                    badge_icon="schedule"
                    secondary_label="View Route"
                    secondary_url="https://www.strava.com/routes/3480270817809452530"
                />
            </section>

            // --- Schedule ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// run schedule"}</span>
                </div>
                <div class="timeline">
                    <div class="timeline-item active">
                        <div class="timeline-month">{"Saturday, April 18"}</div>
                        <div class="timeline-book">
                            <a href="https://www.strava.com/routes/3480270817809452530" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Route →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Upcoming"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Wednesday, April 15"}</div>
                        <div class="timeline-book">
                            <a href="https://www.strava.com/routes/3478757606243375166" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Route →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past run"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Saturday, April 11"}</div>
                        <div class="timeline-book">
                            <a href="https://www.strava.com/routes/3476595695882355858" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Route →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past run"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Wednesday, April 8"}</div>
                        <div class="timeline-book">
                            <a href="https://www.strava.com/routes/3476735579330903130" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Route →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past run"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Saturday, April 4"}</div>
                        <div class="timeline-book">
                            <a href="https://www.strava.com/routes/3475160646034726692" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Route →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past run"}</div>
                    </div>
                </div>
            </section>

            // --- Info Cards ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// get involved"}</span>
                </div>
                <div class="info-grid">
                    <a class="info-card" href="https://www.strava.com/athletes/161795832" target="_blank" rel="noopener" style="text-decoration:none;color:inherit;">
                        <span class="info-icon material-symbols-outlined">{"directions_run"}</span>
                        <h3>{"Follow on Strava"}</h3>
                        <p>{"See routes, track runs, and follow along with the crew."}</p>
                    </a>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"calendar_month"}</span>
                        <h3>{"Every Wed & Sat"}</h3>
                        <p>{"We run twice a week. Show up, no sign-up required."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"social_leaderboard"}</span>
                        <h3>{"All Paces Welcome"}</h3>
                        <p>{"Whether you're training for a race or just getting started — come run with us."}</p>
                    </div>
                </div>
            </section>
        </>
    }
}
