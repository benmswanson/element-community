use yew::prelude::*;

use crate::community::{RUN_CLUB_FEATURED, RUN_CLUB_TIMELINE};
use crate::components::{event_card::EventCard, section::Section, timeline::Timeline};

#[function_component(RunClub)]
pub fn run_club() -> Html {
    html! {
        <>
            <Section
                eyebrow="Next Run"
                title=""
                description=""
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
                eyebrow="Previous Routes"
                title=""
                description=""
            >
                <Timeline items={RUN_CLUB_TIMELINE.to_vec()} />
            </Section>

        </>
    }
}
