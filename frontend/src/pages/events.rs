use yew::prelude::*;

use crate::community::{COMMUNITY_EVENT_FEATURED, COMMUNITY_EVENTS_INFO, COMMUNITY_EVENTS_TIMELINE};
use crate::components::{cta_button::CtaButton, event_card::EventCard, hero::Hero, section::Section, timeline::Timeline};

#[function_component(Events)]
pub fn events() -> Html {
    html! {
        <>
            <Hero
                kicker="Community Events"
                title="Socials, one-offs, and everything else that keeps"
                accent={Some(AttrValue::from("Element social."))}
                subtitle="The broader community calendar stays visible here with the same featured event and timeline rhythm as the older site."
            />

            <Section
                eyebrow="Next Event"
                title="The next community event leads the page."
                description="This keeps the most immediate RSVP item front and center, just like the earlier version."
            >
                <EventCard
                    title={COMMUNITY_EVENT_FEATURED.title}
                    date={COMMUNITY_EVENT_FEATURED.date}
                    description={COMMUNITY_EVENT_FEATURED.description}
                    badge={Some(AttrValue::from("Upcoming"))}
                    location={COMMUNITY_EVENT_FEATURED.location.map(AttrValue::from)}
                    image_url={COMMUNITY_EVENT_FEATURED.image_url.map(AttrValue::from)}
                    primary_href={COMMUNITY_EVENT_FEATURED.primary_href.map(AttrValue::from)}
                    primary_label={COMMUNITY_EVENT_FEATURED.primary_label.map(AttrValue::from)}
                    secondary_href={COMMUNITY_EVENT_FEATURED.secondary_href.map(AttrValue::from)}
                    secondary_label={COMMUNITY_EVENT_FEATURED.secondary_label.map(AttrValue::from)}
                    featured=true
                />
            </Section>

            <Section
                eyebrow="Timeline"
                title="Upcoming and past events stay visible."
                description="The event history and upcoming RSVP links remain in a dedicated timeline view."
            >
                <Timeline items={COMMUNITY_EVENTS_TIMELINE.to_vec()} />
            </Section>

            <Section
                eyebrow="Get Involved"
                title="A lighter way into the community."
                description="Socials, special events, and bring-a-friend energy still have their own space."
            >
                <div class="community-links-grid">
                    {COMMUNITY_EVENTS_INFO.iter().map(|card| html! {
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
