use yew::prelude::*;

use crate::components::{cta_button::CtaButton, section::Section};

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <>
            <Section
                eyebrow="Mission"
                title="Meaningful training starts with people"
                description=""
            >
                <div class="narrative-grid">
                    <article class="info-panel">
                        <p class="card-copy">
                            {"Element Training Club is a strength, conditioning, and mobility gym in Williamsburg, Brooklyn. We’re home to a supportive, inclusive, and motivating community centered around team-based workouts."}
                        </p>
                    </article>
                    <article class="info-panel">
                        <p class="card-copy">
                            {"Our mission is to build a strong community by delivering balanced programming designed to help our members reach their goals. We offer challenging yet approachable workout classes that support growth and progress for all levels."}
                        </p>
                    </article>
                    <article class="info-panel">
                        <p class="card-copy">
                            {"We started with a mission to make training more meaningful— and more human. We believe training is about showing up for yourself and others, building confidence through consistency, and creating momentum that carries into everyday life."}
                        </p>
                    </article>
                </div>
            </Section>

            <Section
                eyebrow="Visit & Contact"
                title="Stop by, give us a call, ask questions!"
                description=""
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
