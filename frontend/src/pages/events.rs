// frontend/src/pages/events.rs

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
        </>
    }
}
