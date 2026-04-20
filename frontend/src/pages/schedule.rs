use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::components::{cta_button::CtaButton, hero::Hero, section::Section};

#[function_component(Schedule)]
pub fn schedule() -> Html {
    html! {
        <>
            <Hero
                kicker="Schedule"
                title="A simple way to"
                accent={Some(AttrValue::from("get in the room."))}
                subtitle="Use the live schedule to browse classes, book sessions, and find the format that matches your week."
            >
                <CtaButton
                    href="https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=-7&sTG=23&sVT=517&sView=day&sLoc=0"
                    label="Open live schedule"
                    primary=true
                />
                <CtaButton
                    href="https://apps.apple.com/us/app/element-training-club/id6743843274"
                    label="Download the app"
                />
            </Hero>

            <Section
                eyebrow="How it works"
                title="Browse classes, choose your time, and book directly."
                description="Scheduling remains powered by Element’s existing booking flow, so members can use the same trusted system for class reservations and account management."
            >
                <div class="feature-grid">
                    <article class="feature-card">
                        <p class="card-label">{"01"}</p>
                        <h3>{"See what’s running"}</h3>
                        <p>{"Check the live schedule for current class times and availability."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"02"}</p>
                        <h3>{"Match the class format"}</h3>
                        <p>{"Use the classes page if you want a quick overview before choosing your first session."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"03"}</p>
                        <h3>{"Book in the existing system"}</h3>
                        <p>{"Reservations, account actions, and purchases continue through the current booking platform."}</p>
                    </article>
                </div>
            </Section>

            <Section
                eyebrow="Need context?"
                title="Not sure which class to start with?"
                description="If you want a quick read on the training formats before booking, head to the classes page and see how strength, conditioning, and Hyrox sessions differ."
            >
                <div class="inline-actions">
                    <Link<Route> to={Route::Classes} classes={classes!("cta-button", "primary")}>
                        {"Review class formats"}
                    </Link<Route>>
                    <CtaButton href="mailto:blake@element-training.com" label="Ask a question" />
                </div>
            </Section>
        </>
    }
}
