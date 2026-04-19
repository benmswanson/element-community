use yew::prelude::*;

use crate::components::{hero::Hero, pricing_card::PricingCard, section::Section};

struct Plan {
    title: &'static str,
    price: &'static str,
    cadence: &'static str,
    cta_href: &'static str,
    cta_label: &'static str,
    badge: Option<&'static str>,
    bullets: &'static [&'static str],
    featured: bool,
}

const PLANS: [Plan; 4] = [
    Plan {
        title: "Unlimited First Week Intro Offer",
        price: "$65",
        cadence: "All classes • 7 days after first visit",
        cta_href: "https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=41&sTG=23&prodId=10313",
        cta_label: "Get started",
        badge: Some("Best value"),
        bullets: &[
            "Try all our class formats for a full week",
            "Meet the coaches and learn the flow of our space",
            "Best introduction to the community",
        ],
        featured: true,
    },
    Plan {
        title: "Unlimited Classes Membership",
        price: "$275 / month",
        cadence: "All classes • Monthly membership",
        cta_href: "https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=40&prodId=10007",
        cta_label: "Buy membership",
        badge: None,
        bullets: &[
            "Full access to every class type",
            "Great for building consistency",
            "Includes community and studio events",
        ],
        featured: false,
    },
    Plan {
        title: "$36 Drop-In",
        price: "$36",
        cadence: "1 class",
        cta_href: "https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=-7&sTG=23&sVT=517&sView=day&sLoc=0",
        cta_label: "Book a class",
        badge: None,
        bullets: &[
            "Perfect for visiting",
            "Great for trying one workout",
            "Works with any class format",
        ],
        featured: false,
    },
    Plan {
        title: "Class Packs",
        price: "$105 / $330",
        cadence: "3-class or 10-class pack",
        cta_href: "https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=42",
        cta_label: "Buy a pack",
        badge: None,
        bullets: &[
            "3-class pack for flexible schedules",
            "10-class pack for steady momentum",
            "Use any option for any class",
        ],
        featured: false,
    },
];

#[function_component(Pricing)]
pub fn pricing() -> Html {
    html! {
        <>
            <Hero
                kicker="Pricing"
                title="Simple memberships built for"
                accent={Some(AttrValue::from("consistent training."))}
                subtitle="Whether you want a flexible drop-in or full access, every option is designed to make getting into class straightforward."
            />

            <Section
                eyebrow="Memberships & Pricing"
                title="Choose the option that fits your rhythm."
                description="All plans can be used across class types. Full details continue to live in the existing checkout flow."
            >
                <div class="pricing-grid">
                    {PLANS.iter().map(|plan| html! {
                        <PricingCard
                            title={plan.title}
                            price={plan.price}
                            cadence={plan.cadence}
                            cta_href={plan.cta_href}
                            cta_label={plan.cta_label}
                            badge={plan.badge.map(AttrValue::from)}
                            bullets={plan.bullets.iter().map(|item| AttrValue::from(*item)).collect::<Vec<_>>()}
                            featured={plan.featured}
                        />
                    }).collect::<Html>()}
                </div>
            </Section>

            <Section
                eyebrow="Questions"
                title="Have a unique schedule or visiting needs?"
                description="If you are dropping in occasionally or want help choosing the right option, reach out directly."
            >
                <div class="visit-grid">
                    <article class="info-panel">
                        <p class="card-label">{"Email"}</p>
                        <h3>{"blake@element-training.com"}</h3>
                        <p class="card-copy">{"Questions about memberships, drop-ins, or where to begin are always welcome."}</p>
                    </article>
                </div>
            </Section>
        </>
    }
}
