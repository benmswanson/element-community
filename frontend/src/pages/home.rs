use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::community::{BOOK_CLUB_CURRENT, COMMUNITY_EVENT_FEATURED, RUN_CLUB_FEATURED};
use crate::components::{
    book_card::BookCard, cta_button::CtaButton, event_card::EventCard, pricing_card::PricingCard,
    section::Section,
};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            <section class="page-hero">
                <div class="shell-container page-hero-inner home-hero-layout">
                        <video id="hero-bg-video" class="hero-card-video" autoplay=true muted=true loop=true playsinline=true />
                    <div class="page-hero-copy">
                        <p class="page-kicker">{"Williamsburg, Brooklyn"}</p>
                        <h1>{"We all go further when we "} <span>{"train together."}</span></h1>
                        <p class="page-subtitle">
                            {"Coach-led strength, conditioning, and mobility classes built around a welcoming team environment."}
                        </p>
                        <div class="page-hero-actions">
                            <CtaButton
                                href="https://www.element-training.com/schedule"
                                label="See the class schedule"
                                light=true
                            />
                            <CtaButton
                                href="https://apps.apple.com/us/app/element-training-club/id6743843274"
                                label="Download the app"
                                light=true
                            />
                        </div>
                    </div>
                    <div class="hero-intro-card">
                        <PricingCard
                            title="Unlimited First Week Intro Offer"
                            price="$65"
                            cadence="All classes • 7 days after first visit"
                            cta_href="https://clients.mindbodyonline.com/ASP/main_shop.asp?studioid=5743626&tg=&vt=&lvl=&stype=43&view=&trn=0&page=&catid=&prodid=100050&date=4%2f20%2f2026&classid=0&prodGroupId=&sSU=&optForwardingLink=&qParam=&justloggedin=&nLgIn=&pMode=3&loc=1"
                            cta_label="Claim intro offer"
                            badge={Some(AttrValue::from("Best way to start"))}
                            bullets={vec![
                                AttrValue::from("Try all our class formats for a full week"),
                                AttrValue::from("Meet the coaches and learn the flow of the space"),
                                AttrValue::from("The easiest entry point into the community"),
                            ]}
                            featured=true
                        />
                    </div>
                </div>
            </section>

            <Section
                eyebrow="About"
                title="Meaningful training starts with people"
                description=""
            >
                <div class="narrative-grid">
                    <article class="info-panel">
                        <p class="card-copy">
                            {"Element is a strength, conditioning, and mobility gym in Williamsburg, Brooklyn. We train in groups, the energy in the room is part of what makes it work."}
                        </p>
                    </article>
                    <article class="info-panel">
                        <p class="card-copy">
                            {"The programming is built to be challenging without being intimidating. Whether you’re just getting started or you’ve been training for years, there’s room to push and room to grow."}
                        </p>
                    </article>
                    <article class="info-panel">
                        <p class="card-copy">
                            {"We believe that training is better with community, and our events outside of classes are there to build connection. Check out our run club, book club, and other community events!"}
                        </p>
                    </article>
                </div>
            </Section>

            <Section
                eyebrow="Upcoming"
                title="What’s happening in the community right now"
                description=""
            >
                <div class="community-preview-grid">
                    <EventCard
                        title={RUN_CLUB_FEATURED.title}
                        date={RUN_CLUB_FEATURED.date}
                        description={RUN_CLUB_FEATURED.description}
                        badge={Some(AttrValue::from("Run Club"))}
                        location={RUN_CLUB_FEATURED.location.map(AttrValue::from)}
                        image_url={RUN_CLUB_FEATURED.image_url.map(AttrValue::from)}
                        primary_href={RUN_CLUB_FEATURED.primary_href.map(AttrValue::from)}
                        primary_label={RUN_CLUB_FEATURED.primary_label.map(AttrValue::from)}
                        secondary_href={RUN_CLUB_FEATURED.secondary_href.map(AttrValue::from)}
                        secondary_label={RUN_CLUB_FEATURED.secondary_label.map(AttrValue::from)}
                        detail_route={Some(Route::RunClub)}
                        detail_label={Some(AttrValue::from("See Run Club"))}
                    />
                    <EventCard
                        title={COMMUNITY_EVENT_FEATURED.title}
                        date={COMMUNITY_EVENT_FEATURED.date}
                        description={COMMUNITY_EVENT_FEATURED.description}
                        badge={Some(AttrValue::from("Community Event"))}
                        location={COMMUNITY_EVENT_FEATURED.location.map(AttrValue::from)}
                        image_url={COMMUNITY_EVENT_FEATURED.image_url.map(AttrValue::from)}
                        primary_href={COMMUNITY_EVENT_FEATURED.primary_href.map(AttrValue::from)}
                        primary_label={COMMUNITY_EVENT_FEATURED.primary_label.map(AttrValue::from)}
                        secondary_href={COMMUNITY_EVENT_FEATURED.secondary_href.map(AttrValue::from)}
                        secondary_label={COMMUNITY_EVENT_FEATURED.secondary_label.map(AttrValue::from)}
                        detail_route={Some(Route::Events)}
                        detail_label={Some(AttrValue::from("See Community Events"))}
                    />
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
                        detail_route={Some(Route::BookClub)}
                        detail_label={Some(AttrValue::from("See Book Club"))}
                    />
                </div>
            </Section>


            <div class="shell-container home-contact-tagline">
                <p>{"Stop by or give us a call, we'd love to chat!"}</p>
            </div>

            <div class="shell-container home-contact-card">
                <p class="home-contact-address">{"667 Grand Street, Brooklyn, NY 11211"}</p>
                <div class="home-contact-buttons">
                    <a class="home-contact-btn" href="tel:+19547785650">{"Call Us"}</a>
                    <a class="home-contact-btn" href="mailto:blake@element-training.com">{"Email Us"}</a>
                </div>
            </div>

        </>
    }
}
