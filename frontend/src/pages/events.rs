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

            // --- Coming Soon ---
            <section class="section">
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
                        <span class="info-icon material-symbols-outlined">{"more_horiz"}</span>
                        <h3>{"More Coming Soon"}</h3>
                        <p>{"Stay tuned — more events are on the way."}</p>
                    </div>
                </div>
            </section>
        </>
    }
}
