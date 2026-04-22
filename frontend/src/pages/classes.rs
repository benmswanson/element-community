use yew::prelude::*;

use crate::components::{class_card::ClassCard, section::Section};

struct ClassInfo {
    title: &'static str,
    description: &'static str,
    bullets: &'static [&'static str],
}

const CLASSES: [ClassInfo; 3] = [
    ClassInfo {
        title: "Strength & Conditioning",
        description: "Circuit-style training that blends strength and cardio in formats that keep the room moving together at different intensities.",
        bullets: &[
            "Functional movement patterns and interval work",
            "A mix of strength and endurance challenges",
            "Designed to stay approachable while still pushing effort",
        ],
    },
    ClassInfo {
        title: "Hyrox Training",
        description: "Hybrid sessions built around Hyrox-style movements and efforts, combining structured strength and conditioning work.",
        bullets: &[
            "Longer efforts, repeat intervals, and race-style formats",
            "Partner and team elements throughout training",
            "Useful for race prep and still beginner-friendly",
        ],
    },
    ClassInfo {
        title: "Strength",
        description: "Build strength through barbell lifts and accessory work with a focus on form, progression, and consistency.",
        bullets: &[
            "Lower body emphasis with squats, lunges, and posterior chain work",
            "Upper body pressing and pulling sessions",
            "Full-body training days that tie the work together",
        ],
    },
];

#[function_component(Classes)]
pub fn classes() -> Html {
    html! {
        <>
            <Section
                eyebrow=""
                title="Our Classes"
                description=""
            >
                <div class="class-grid">
                    {CLASSES.iter().map(|item| html! {
                        <ClassCard
                            title={item.title}
                            description={item.description}
                            bullets={item.bullets.iter().map(|bullet| AttrValue::from(*bullet)).collect::<Vec<_>>()}
                        />
                    }).collect::<Html>()}
                </div>
            </Section>

            <Section
                eyebrow="Expectations"
                title="Structured coaching without guesswork"
                description="You choose your own weights, move at your own pace, and get guidance throughout class from a coach who is there to help you train well."
            >
                <div class="feature-grid">
                    <article class="feature-card">
                        <p class="card-label">{"Coach-led"}</p>
                        <h3>{"Clear instruction"}</h3>
                        <p>{"Every class is led by a coach, not left to screens or self-guided stations."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"Approachable"}</p>
                        <h3>{"All levels welcome"}</h3>
                        <p>{"Members can scale effort, choose weights, and learn the flow without feeling out of place."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"Progressive"}</p>
                        <h3>{"Built to keep you improving"}</h3>
                        <p>{"Programming is varied enough to stay engaging and structured enough to build momentum over time."}</p>
                    </article>
                </div>
            </Section>
        </>
    }
}
