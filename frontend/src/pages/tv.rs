use yew::prelude::*;

struct TvCard {
    label: &'static str,
    title: &'static str,
    date: &'static str,
    image_url: &'static str,
    href: Option<&'static str>,
    btn_label: &'static str,
}

const TV_CARDS: [TvCard; 3] = [
    TvCard {
        label: "Book Club",
        title: "Headshot",
        date: "April 30 · 8:00 PM",
        image_url: "https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg",
        href: Some("https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"),
        btn_label: "RSVP",
    },
    TvCard {
        label: "Community Event",
        title: "Good Saturdays: The Social",
        date: "May 2 · 2:00 PM",
        image_url: "https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FUHRiYw9aKMeD6W6RofPbz7sjeMx1%2FTIhQ5EdeplVHWC52RZJbm?alt=media&token=abaad7ae-1a79-4421-9c7e-9cf7f30a7e84",
        href: Some("https://partiful.com/e/VoAlpsKNThTgyprJaSIU"),
        btn_label: "RSVP",
    },
    TvCard {
        label: "Saturday Run · 9:00 AM",
        title: "A Park Named Maria",
        date: "May 3 · 9:00 AM",
        image_url: "https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OFIDOJU4DG24FLBVI2BE3AAXF5442KVELW42E53YNN3SCMJPXEDKAYMUEP34UFYFE2WOA6ZH2GEPMT2YDTDDMSCDQI74GD7LL7Q2ZG3ATTNEI55LAATLDVRVXZ4GYUHOHBWSJK3ZVXHEMCWUXPEQHHYN5OY3DWGPI5HQGZMYSZUXSU3Q7IXLUSHUV2RKNVPKOU======",
        href: Some("https://www.strava.com/routes/3484708461539968178"),
        btn_label: "View Route",
    },
];

#[function_component(TvDisplay)]
pub fn tv_display() -> Html {
    html! {
        <div class="tv-page">
            <div class="tv-header">
                <a href="/"><img src="/assets/etc-logo-white.png" class="tv-logo" alt="Element" /></a>
                <span class="tv-heading">{"What's Coming Up"}</span>
            </div>

            <div class="tv-upcoming-grid">
                { TV_CARDS.iter().map(|card| html! {
                    <div class="tv-card-wrapper">
                        <div class="tv-card-date-bar">
                            <span class="tv-card-date-label">{card.label}</span>
                            <span class="tv-card-date-text">{card.date}</span>
                        </div>
                        <div class="tv-event-card">
                            <img src={card.image_url} alt={card.title} class="tv-event-card-img" />
                            <div class="tv-event-card-overlay">
                                <h2 class="tv-event-card-title">{card.title}</h2>
                                if let Some(href) = card.href {
                                    <a href={href} class="tv-rsvp-btn" target="_blank">{card.btn_label}</a>
                                }
                            </div>
                        </div>
                    </div>
                }).collect::<Html>() }
            </div>
        </div>
    }
}
