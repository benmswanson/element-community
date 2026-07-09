use yew::prelude::*;

struct TvCard {
    label: &'static str,
    title: &'static str,
    date: &'static str,
    image_url: &'static str,
    href: Option<&'static str>,
    btn_label: &'static str,
    rotation: &'static str,
    pin_color: &'static str,
    img_position: &'static str,
    img_fit: &'static str,
}

const TV_CARDS: [TvCard; 3] = [
    TvCard {
        label: "Community Event",
        title: "Good Saturdays 04",
        date: "July 11 · 12:00 PM",
        image_url: "https://ilove.sweatpals.com/api/files/3471db1b-7ac3-47ad-90ea-647efce13865?variant=l",
        href: Some("https://sweatpals.com/event/good-saturdays-03-the-workout-presented-by-element-training-club-x-ralle-e5ce4/2026-07-11"),
        btn_label: "Register",
        rotation: "-1deg",
        pin_color: "pin-red",
        img_position: "center",
        img_fit: "cover",
    },
    TvCard {
        label: "Community Event",
        title: "Saturday Social",
        date: "July 18 · 5:00 PM",
        image_url: "https://firebasestorage.googleapis.com/v0/b/getpartiful.appspot.com/o/external%2Fuser%2FUHRiYw9aKMeD6W6RofPbz7sjeMx1%2F6zkHLyTzHfEW-bz8Hz7GI?alt=media&token=93d8fadd-c40d-4040-b740-b3a0979f43ff",
        href: Some("https://partiful.com/e/5SZrGeomVoGQzkQ0S79p"),
        btn_label: "RSVP",
        rotation: "1deg",
        pin_color: "pin-green",
        img_position: "bottom",
        img_fit: "cover",
    },
    TvCard {
        label: "Book Club",
        title: "July Book Club",
        date: "July 30 · 8:00 PM",
        image_url: "/assets/klara.jpg",
        href: Some("https://partiful.com/e/ppHQfZX9N1xsuv3WnUsz"),
        btn_label: "RSVP",
        rotation: "-2deg",
        pin_color: "pin-blue",
        img_position: "center",
        img_fit: "cover",
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
                        <div class={format!("tv-pin {}", card.pin_color)}></div>
                        <div class="tv-flyer">
                            <div class="tv-flyer-img-wrap">
                                <img src={card.image_url} alt={card.title} class="tv-flyer-img" style={format!("object-position: {}; object-fit: {}", card.img_position, card.img_fit)} />
                            </div>
                            <div class="tv-flyer-body">
                                <h2 class="tv-flyer-title">{card.title}</h2>
                                <p class="tv-flyer-date">{card.date}</p>
                            </div>
                        </div>
                    </div>
                }).collect::<Html>() }
            </div>

            <div class="tv-board-footer">
                <div class="tv-footer-left">
                    <div class="tv-footer-pair">
                        <div class="tv-qr-postit">
                            <div class="tv-pin pin-green"></div>
                            <img
                                src="https://api.qrserver.com/v1/create-qr-code/?size=120x120&data=https%3A%2F%2Fchat.whatsapp.com%2FKfoam7bTX0K9obgDVC8QXP%3Fmode%3Dgi_t&color=000000&bgcolor=dce8ff"
                                alt="WhatsApp QR Code"
                                class="tv-qr-code"
                            />
                        </div>
                        <div class="tv-text-postit" style="transform: rotate(1.5deg)">
                            <div class="tv-pin pin-green"></div>
                            <p class="tv-postit-label">{"WhatsApp"}<br/>{"Community"}</p>
                        </div>
                    </div>
                </div>
                <div class="tv-footer-center">
                    <div class="tv-logo-postit">
                        <div class="tv-pin pin-yellow"></div>
                        <a href="/"><img src="/assets/etc-logo-blue.png" class="tv-logo" alt="Element" /></a>
                    </div>
                </div>
                <div class="tv-footer-right">
                    <div class="tv-footer-pair">
                        <div class="tv-qr-postit">
                            <div class="tv-pin pin-blue"></div>
                            <img
                                src="https://api.qrserver.com/v1/create-qr-code/?size=120x120&data=https%3A%2F%2Felement-community.com&color=000000&bgcolor=dce8ff"
                                alt="element-community.com QR Code"
                                class="tv-qr-code"
                            />
                        </div>
                        <div class="tv-text-postit" style="transform: rotate(-1deg)">
                            <div class="tv-pin pin-blue"></div>
                            <p class="tv-postit-label">{"All Event"}<br/>{"Details"}</p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
