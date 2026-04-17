// frontend/src/pages/home.rs

use crate::app::Route;
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

            <section class="section">
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
