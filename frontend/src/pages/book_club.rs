use yew::prelude::*;

use crate::community::BOOK_CLUB_CURRENT;
use crate::components::{book_card::BookCard, cta_button::CtaButton, section::Section};

#[function_component(BookClub)]
pub fn book_club() -> Html {
    html! {
        <>
            <Section
                eyebrow="Currently Reading"
                title=""
                description=""
            >
                <BookCard
                    title={BOOK_CLUB_CURRENT.title}
                    author={BOOK_CLUB_CURRENT.author}
                    description={BOOK_CLUB_CURRENT.description}
                    cover_url={BOOK_CLUB_CURRENT.cover_url}
                    month={BOOK_CLUB_CURRENT.month}
                    badge={Some(AttrValue::from("Reading Now"))}
                    primary_href={BOOK_CLUB_CURRENT.primary_href.map(AttrValue::from)}
                    primary_label={BOOK_CLUB_CURRENT.primary_label.map(AttrValue::from)}
                    secondary_href={BOOK_CLUB_CURRENT.secondary_href.map(AttrValue::from)}
                    secondary_label={BOOK_CLUB_CURRENT.secondary_label.map(AttrValue::from)}
                    featured=true
                />
            </Section>

<Section
                eyebrow="Get Involved"
                title="Have a pick for the next month?"
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
