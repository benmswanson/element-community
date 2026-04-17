// frontend/src/pages/run_club.rs

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

            // --- Run Schedule ---
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

            // --- Strava CTA ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// find us on strava"}</span>
                </div>
                <div class="card-actions" style="border-top: none; padding-top: 0;">
                    <a class="btn btn-primary" href="https://www.strava.com/athletes/161795832" target="_blank" rel="noopener">
                        <span class="material-symbols-outlined">{"directions_run"}</span>
                        {"Follow on Strava"}
                    </a>
                </div>
            </section>
        </>
    }
}
