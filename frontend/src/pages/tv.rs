use yew::prelude::*;

struct TvCard {
    label: &'static str,
    title: &'static str,
    date: &'static str,
    image_url: &'static str,
    href: Option<&'static str>,
    btn_label: &'static str,
    rotation: &'static str,
}

const TV_CARDS: [TvCard; 3] = [
    TvCard {
        label: "Book Club",
        title: "Headshot",
        date: "April 30 · 8:00 PM",
        image_url: "https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg",
        href: Some("https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"),
        btn_label: "RSVP",
        rotation: "-2deg",
    },
    TvCard {
        label: "Community Event",
        title: "Good Saturdays: The Social",
        date: "May 2 · 2:00 PM",
        image_url: "https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FUHRiYw9aKMeD6W6RofPbz7sjeMx1%2FTIhQ5EdeplVHWC52RZJbm?alt=media&token=abaad7ae-1a79-4421-9c7e-9cf7f30a7e84",
        href: Some("https://partiful.com/e/VoAlpsKNThTgyprJaSIU"),
        btn_label: "RSVP",
        rotation: "1deg",
    },
    TvCard {
        label: "Run Club",
        title: "A Park Named Maria",
        date: "May 3 · 9:00 AM",
        image_url: "https://d3o5xota0a1fcr.cloudfront.net/v6/maps/OFIDOJU4DG24FLBVI2BE3AAXF5442KVELW42E53YNN3SCMJPXEDKAYMUEP34UFYFE2WOA6ZH2GEPMT2YDTDDMSCDQI74GD7LL7Q2ZG3ATTNEI55LAATLDVRVXZ4GYUHOHBWSJK3ZVXHEMCWUXPEQHHYN5OY3DWGPI5HQGZMYSZUXSU3Q7IXLUSHUV2RKNVPKOU======",
        href: Some("https://www.strava.com/routes/3484708461539968178"),
        btn_label: "View Route",
        rotation: "-1deg",
    },
];

#[function_component(TvDisplay)]
pub fn tv_display() -> Html {
    html! {
        <div class="tv-page tv-board">
            <div class="tv-board-header">
                <span class="tv-heading">{"What's Coming Up"}</span>
            </div>

            <div class="tv-upcoming-grid">
                { TV_CARDS.iter().map(|card| html! {
                    <div class="tv-flyer-wrapper" style={format!("transform: rotate({})", card.rotation)}>
                        <div class="tv-pin"></div>
                        <div class="tv-flyer">
                            <div class="tv-flyer-img-wrap">
                                <img src={card.image_url} alt={card.title} class="tv-flyer-img" />
                            </div>
                            <div class="tv-flyer-body">
                                <p class="tv-flyer-label">{card.label}</p>
                                <h2 class="tv-flyer-title">{card.title}</h2>
                                <p class="tv-flyer-date">{card.date}</p>
                                if let Some(href) = card.href {
                                    <a href={href} class="tv-flyer-btn" target="_blank">{card.btn_label}</a>
                                }
                            </div>
                        </div>
                    </div>
                }).collect::<Html>() }
            </div>

            <div class="tv-board-footer">
                <div class="tv-qr-center">
                    <div class="tv-qr-postit">
                        <img
                            src="https://api.qrserver.com/v1/create-qr-code/?size=120x120&data=https%3A%2F%2Fchat.whatsapp.com%2FKfoam7bTX0K9obgDVC8QXP%3Fmode%3Dgi_t&color=000000&bgcolor=fde84a"
                            alt="WhatsApp QR Code"
                            class="tv-qr-code"
                        />
                        <p class="tv-qr-sub">{"Scan to join"}</p>
                    </div>
                    <div class="tv-qr-note">
                        <p class="tv-qr-label">{"Join our WhatsApp community!"}</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
