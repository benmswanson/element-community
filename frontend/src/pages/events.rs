use yew::prelude::*;

use crate::community::{BOOK_CLUB_CURRENT, BOOK_CLUB_TIMELINE, COMMUNITY_EVENT_FEATURED, COMMUNITY_EVENTS_TIMELINE};
use crate::components::{
    book_card::BookCard, cta_button::CtaButton, event_card::EventCard, section::Section,
    timeline::Timeline,
};

#[function_component(Events)]
pub fn events() -> Html {
    let past_events = COMMUNITY_EVENTS_TIMELINE
        .iter()
        .filter(|e| !e.active)
        .cloned()
        .collect::<Vec<_>>();

    let past_books = BOOK_CLUB_TIMELINE
        .iter()
        .filter(|e| !e.active)
        .cloned()
        .collect::<Vec<_>>();

    html! {
        <>
            <Section
                eyebrow="What's Coming Up"
                title=""
                description=""
            >
                <EventCard
                    title={COMMUNITY_EVENT_FEATURED.title}
                    date={COMMUNITY_EVENT_FEATURED.date}
                    description={COMMUNITY_EVENT_FEATURED.description}
                    badge={Some(AttrValue::from("Upcoming"))}
                    location={COMMUNITY_EVENT_FEATURED.location.map(AttrValue::from)}
                    image_url={COMMUNITY_EVENT_FEATURED.image_url.map(AttrValue::from)}
                    img_transform={COMMUNITY_EVENT_FEATURED.img_transform.map(AttrValue::from)}
                    primary_href={COMMUNITY_EVENT_FEATURED.primary_href.map(AttrValue::from)}
                    primary_label={COMMUNITY_EVENT_FEATURED.primary_label.map(AttrValue::from)}
                    secondary_href={COMMUNITY_EVENT_FEATURED.secondary_href.map(AttrValue::from)}
                    secondary_label={COMMUNITY_EVENT_FEATURED.secondary_label.map(AttrValue::from)}
                    featured=true
                />
            </Section>

            <Section
                eyebrow="Also Upcoming"
                title=""
                description=""
            >
                <BookCard
                    title={BOOK_CLUB_CURRENT.title}
                    author={BOOK_CLUB_CURRENT.author}
                    description={BOOK_CLUB_CURRENT.description}
                    cover_url={BOOK_CLUB_CURRENT.cover_url}
                    month={BOOK_CLUB_CURRENT.month}
                    badge={Some(AttrValue::from("Book Club"))}
                    primary_href={BOOK_CLUB_CURRENT.primary_href.map(AttrValue::from)}
                    primary_label={BOOK_CLUB_CURRENT.primary_label.map(AttrValue::from)}
                    secondary_href={BOOK_CLUB_CURRENT.secondary_href.map(AttrValue::from)}
                    secondary_label={BOOK_CLUB_CURRENT.secondary_label.map(AttrValue::from)}
                    featured=true
                />
            </Section>

            <Section
                eyebrow="Past Events"
                title=""
                description=""
            >
                <Timeline items={past_events} />
            </Section>

            <Section
                eyebrow="Past Reads"
                title=""
                description=""
            >
                <Timeline items={past_books} />
            </Section>

            <Section
                eyebrow="Book Club"
                title="Have a pick for next month?"
                description=""
            >
                <div class="inline-actions">
                    <CtaButton
                        href="https://www.goodreads.com/group/invite/7267353-element-book-club?invite_token=MmNiYmVkZTYtMWM5ZC00MzMyLThhMWEtMDMxMDUyNTliZDNi&utm_medium=email&utm_source=copypastegroup"
                        label="Join Goodreads Group"
                        primary=true
                    />
                    <CtaButton
                        href="https://forms.gle/eQFo1SqXJwRfr3tX6"
                        label="Suggest a Book"
                    />
                </div>
            </Section>
        </>
    }
}
