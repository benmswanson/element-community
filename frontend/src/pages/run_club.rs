use yew::prelude::*;

use crate::community::{RUN_CLUB_FEATURED, RUN_CLUB_INFO, RUN_CLUB_TIMELINE};
use crate::components::{cta_button::CtaButton, event_card::EventCard, hero::Hero, section::Section, timeline::Timeline};

#[function_component(RunClub)]
pub fn run_club() -> Html {
    html! {
        <>
            <Hero
                kicker="Run Club"
                title="Get moving with the"
                accent={Some(AttrValue::from("Element community."))}
                subtitle="Weekly runs stay light, welcoming, and easy to join, with routes and cadence still visible in the familiar card-and-timeline format."
            />

            <Section
                eyebrow="Next Run"
                title="The next route is ready."
                description="Featured run details still lead the page so people can see the immediate plan first."
            >
                <EventCard
                    title={RUN_CLUB_FEATURED.title}
                    date={RUN_CLUB_FEATURED.date}
                    description={RUN_CLUB_FEATURED.description}
                    badge={Some(AttrValue::from("Upcoming"))}
                    location={RUN_CLUB_FEATURED.location.map(AttrValue::from)}
                    image_url={RUN_CLUB_FEATURED.image_url.map(AttrValue::from)}
                    primary_href={RUN_CLUB_FEATURED.primary_href.map(AttrValue::from)}
                    primary_label={RUN_CLUB_FEATURED.primary_label.map(AttrValue::from)}
                    secondary_href={RUN_CLUB_FEATURED.secondary_href.map(AttrValue::from)}
                    secondary_label={RUN_CLUB_FEATURED.secondary_label.map(AttrValue::from)}
                    featured=true
                />
            </Section>

            <Section
                eyebrow="Timeline"
                title="The run schedule is back."
                description="Routes and recent runs stay visible in the same rolling timeline format as the older site."
            >
                <Timeline items={RUN_CLUB_TIMELINE.to_vec()} />
            </Section>

            <Section
                eyebrow="Get Involved"
                title="Follow along or just show up."
                description="The supporting run-club cards stay intact so people can quickly understand cadence and vibe."
            >
                <div class="community-links-grid">
                    {RUN_CLUB_INFO.iter().map(|card| html! {
                        <article class="info-panel">
                            <p class="card-label">{card.label}</p>
                            <h3>{card.title}</h3>
                            <p class="card-copy">{card.description}</p>
                            if let Some(href) = card.href {
                                <div class="inline-actions">
                                    <CtaButton href={href} label="Open link" primary={card.primary} />
                                </div>
                            }
                        </article>
                    }).collect::<Html>()}
                </div>
            </Section>
        </>
    }
}
