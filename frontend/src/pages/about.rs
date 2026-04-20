use yew::prelude::*;

use crate::components::{cta_button::CtaButton, section::Section};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <Section
                eyebrow="Mission"
                title="Meaningful training starts with people."
                description="Element was built to help members train with intention, consistency, and confidence while feeling supported by the people around them."
            >
                <div class="narrative-grid">
                    <article class="info-panel">
                        <p class="card-copy">
                            {"Our mission is to build a strong community by delivering balanced programming designed to help members reach their goals. Classes are challenging but approachable, with space for both first-timers and experienced athletes to keep growing."}
                        </p>
                    </article>
                    <article class="info-panel">
                        <p class="card-copy">
                            {"Element started with a simple belief: training should help people show up for themselves and for each other. Consistency, confidence, and momentum inside the gym should carry into everyday life."}
                        </p>
                    </article>
                    <article class="info-panel accent-panel">
                        <p class="card-label">{"The difference"}</p>
                        <h3>{"At Element, we train together."}</h3>
                        <p class="card-copy">{"That changes the energy in the room, the accountability people feel, and the progress they’re able to sustain."}</p>
                    </article>
                </div>
            </Section>

            <Section
                eyebrow="Visit & Contact"
                title="Come by, ask questions, and get to know the room."
                description="Whether you are new to strength training or looking for a better long-term home base, the team is easy to reach."
            >
                <div class="visit-grid">
                    <article class="info-panel">
                        <p class="card-label">{"Address"}</p>
                        <h3>{"667 Grand Street"}</h3>
                        <p class="card-copy">{"Brooklyn, NY 11211"}</p>
                    </article>
                    <article class="info-panel">
                        <p class="card-label">{"Contact"}</p>
                        <h3>{"blake@element-training.com"}</h3>
                        <p class="card-copy">{"(954) 778-5650"}</p>
                    </article>
                    <article class="info-panel">
                        <p class="card-label">{"Follow"}</p>
                        <h3>{"Stay connected"}</h3>
                        <div class="inline-actions">
                            <CtaButton href="https://www.instagram.com/elementtrainingclub/" label="Instagram" primary=true />
                            <CtaButton href="https://open.spotify.com/" label="Spotify" />
                        </div>
                    </article>
                </div>
            </Section>
        </>
    }
}
