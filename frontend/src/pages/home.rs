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
                    <div class="page-hero-copy">
                        <p class="page-kicker">{"Williamsburg, Brooklyn"}</p>
                        <h1>{"We all go further when we "} <span>{"train together."}</span></h1>
                        <p class="page-subtitle">
                            {"Coach-led strength, conditioning, and mobility classes built around a welcoming team environment."}
                        </p>
                        <div class="page-hero-actions">
                            <CtaButton
                                href="https://clients.mindbodyonline.com/ASP/main_shop.asp?studioid=5743626&tg=&vt=&lvl=&stype=43&view=&trn=0&page=&catid=&prodid=100050&date=4%2f20%2f2026&classid=0&prodGroupId=&sSU=&optForwardingLink=&qParam=&justloggedin=&nLgIn=&pMode=3&loc=1"
                                label="Start with the intro offer"
                                primary=true
                            />
                            <CtaButton
                                href="https://www.element-training.com/schedule"
                                label="See the class schedule"
                            />
                        </div>
                    </div>
                </div>
            </section>

            <Section
                eyebrow="Upcoming"
                title="What’s happening in the community right now."
                description="See the next run, the next community event, and the current book club pick without leaving the landing page."
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

<Section
                eyebrow="Why Element"
                title="Training that balances structure, challenge, and community."
                description="Element combines clear coaching, team energy, and smart programming so members can build strength and momentum that lasts."
            >
                <div class="feature-grid">
                    <article class="feature-card">
                        <p class="card-label">{"01"}</p>
                        <h3>{"Team-based format"}</h3>
                        <p>{"Training is a team sport at Element. The group format creates accountability, support, and the energy that keeps people coming back."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"02"}</p>
                        <h3>{"Balanced training"}</h3>
                        <p>{"Programming blends strength, conditioning, and mobility so progress feels sustainable and well-rounded instead of one-dimensional."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"03"}</p>
                        <h3>{"Functionality first"}</h3>
                        <p>{"Classes are designed to help members move better, get stronger, and build confidence that carries into daily life."}</p>
                    </article>
                    <article class="feature-card">
                        <p class="card-label">{"04"}</p>
                        <h3>{"Expert coaching"}</h3>
                        <p>{"Experienced coaches guide every session with clear instruction, welcoming energy, and an emphasis on progress for all levels."}</p>
                    </article>
                </div>
            </Section>

            <Section
                eyebrow="New Here"
                title="Try your first week of unlimited classes for $65."
                description="Meet the team, experience the training style, and explore every class format with a one-week intro offer."
            >
                <div class="split-band">
                    <div class="split-band-copy">
                        <ul class="detail-list">
                            <li>{"7 days start after your first visit"}</li>
                            <li>{"Unlimited access to all class formats"}</li>
                            <li>{"No pressure and no long-term commitment"}</li>
                        </ul>
                    </div>
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
            </Section>

            <Section
                eyebrow="Visit"
                title="Train in the heart of Williamsburg."
                description="Element Training Club is located at 667 Grand Street in Brooklyn, with a schedule designed to make coach-led group training easy to fit into real life."
            >
                <div class="visit-grid">
                    <article class="info-panel">
                        <p class="card-label">{"Address"}</p>
                        <h3>{"Element Training Club"}</h3>
                        <p class="card-copy">{"667 Grand Street"}</p>
                        <p class="card-copy">{"Brooklyn, NY 11211"}</p>
                    </article>
                    <article class="info-panel">
                        <p class="card-label">{"Next step"}</p>
                        <h3>{"Book your first class"}</h3>
                        <p class="card-copy">{"Browse the live schedule, choose a format, and get into the room with the team."}</p>
                        <div class="inline-actions">
                            <CtaButton
                                href="https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=-7&sTG=23&sVT=517&sView=day&sLoc=0"
                                label="Open schedule"
                                primary=true
                            />
                            <CtaButton
                                href="https://apps.apple.com/us/app/element-training-club/id6743843274"
                                label="Download the app"
                            />
                        </div>
                    </article>
                </div>
            </Section>
        </>
    }
}
