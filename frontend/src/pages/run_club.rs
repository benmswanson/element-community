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

            // --- Strava ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// find us on strava"}</span>
                </div>
                <div class="info-grid">
                    <a class="info-card" href="https://www.strava.com/athletes/161795832" target="_blank" rel="noopener" style="text-decoration:none;color:inherit;">
                        <span class="info-icon material-symbols-outlined">{"directions_run"}</span>
                        <h3>{"Strava Profile"}</h3>
                        <p>{"Follow along, see upcoming group runs, and track your miles with the crew."}</p>
                    </a>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"calendar_month"}</span>
                        <h3>{"Upcoming Runs"}</h3>
                        <p>{"Check the community events tab for scheduled group runs and meetups."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"social_leaderboard"}</span>
                        <h3>{"All levels welcome"}</h3>
                        <p>{"Whether you're training for a race or just getting started — come run with us."}</p>
                    </div>
                </div>
            </section>

            // --- CTA ---
            <section class="section">
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
