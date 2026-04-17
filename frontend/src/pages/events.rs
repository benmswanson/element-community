// frontend/src/pages/events.rs

use crate::components::event_card::EventCard;
use yew::prelude::*;

#[function_component(Events)]
pub fn events() -> Html {
    html! {
        <>
            // --- Hero ---
            <section class="hero">
                <h1>{"Community "}<span class="hero-accent">{"Events"}</span></h1>
                <p class="hero-subtitle">
                    {"Socials, one-offs, and everything else Element"}
                </p>
            </section>

            // --- Next Event ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// next event"}</span>
                </div>
                <EventCard
                    title="Element Community Event"
                    date="Saturday, April 18"
                    description="Join the Element community this Saturday. Click RSVP to see details and let us know you're coming."
                    icon="celebration"
                    badge_label="Upcoming"
                    badge_icon="schedule"
                    rsvp_url="https://withforte.co/events/wdsbGbuxMw6ynjLMrcHj"
                />
            </section>

            // --- Timeline ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// events"}</span>
                </div>
                <div class="timeline">
                    <div class="timeline-item active">
                        <div class="timeline-month">{"Saturday, April 18"}</div>
                        <div class="timeline-book">
                            <a href="https://withforte.co/events/wdsbGbuxMw6ynjLMrcHj" target="_blank" rel="noopener" style="color:inherit;">
                                {"RSVP →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Upcoming"}</div>
                    </div>
                    <div class="timeline-item active">
                        <div class="timeline-month">{"Friday, April 25"}</div>
                        <div class="timeline-book">
                            <a href="https://partiful.com/e/eK4W7NufhtWs20oXbeQV" target="_blank" rel="noopener" style="color:inherit;">
                                {"RSVP →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Upcoming"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Saturday, April 11"}</div>
                        <div class="timeline-book">
                            <a href="https://www.rallemovements.com/event-details-registration/ralle-element-training-club-run-hyrox-or-sculpt-social" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Event →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past event"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Friday, March 14"}</div>
                        <div class="timeline-book">
                            <a href="https://partiful.com/e/MheGKxIXEhqwMXeF1fJh" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Event →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past event"}</div>
                    </div>
                    <div class="timeline-item">
                        <div class="timeline-month">{"Wednesday, February 19"}</div>
                        <div class="timeline-book">
                            <a href="https://www.rallemovements.com/event-details-registration/ralle-element-training-club-run-hyrox-social?fbclid=PAQ0xDSwQELiRleHRuA2FlbQIxMQBzcnRjBmFwcF9pZA8xMjQwMjQ1NzQyODc0MTQAAafflu5JWBPwhIg5HDZRpvtbnwKAeLQSfcFwTX9HOYSNDeTKuXJfTJcWZ9Gykw_aem_Vvg5jiBZM5klee0MdU_6rg" target="_blank" rel="noopener" style="color:inherit;">
                                {"View Event →"}
                            </a>
                        </div>
                        <div class="timeline-author">{"Past event"}</div>
                    </div>
                </div>
            </section>

            // --- Info Cards ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// get involved"}</span>
                </div>
                <div class="info-grid">
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"celebration"}</span>
                        <h3>{"Socials"}</h3>
                        <p>{"Post-workout hangs, happy hours, and community get-togethers."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"fitness_center"}</span>
                        <h3>{"Fitness Events"}</h3>
                        <p>{"Races, group workouts, and challenges open to the whole community."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"group"}</span>
                        <h3>{"Bring a Friend"}</h3>
                        <p>{"All events are open — the more the merrier. Tag someone in."}</p>
                    </div>
                </div>
            </section>
        </>
    }
}
