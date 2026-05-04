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
        label: "Run Club",
        title: "Weds Night Track",
        date: "May 6 · 7:00 PM",
        image_url: "https://ilove.sweatpals.com/api/files/9b735b97-c4a3-400e-ad3c-3306ce357957?variant=l",
        href: Some("https://sweatpals.com/event/ralle-track-group/2026-05-06"),
        btn_label: "Register",
        rotation: "-1deg",
        pin_color: "pin-green",
        img_position: "center",
        img_fit: "cover",
    },
    TvCard {
        label: "Community Event",
        title: "HYROX Half Sim",
        date: "May 9 · 667 Grand St",
        image_url: "/assets/sim.jpeg",
        href: Some("https://docs.google.com/spreadsheets/d/1uqfHxSrWA6c35zo59BU4l4MNw2badp695jgBYMOnhi8/edit?gid=1161720329#gid=1161720329"),
        btn_label: "Sign Up",
        rotation: "1deg",
        pin_color: "pin-red",
        img_position: "top",
        img_fit: "cover",
    },
    TvCard {
        label: "Book Club",
        title: "Hamnet",
        date: "May 28 · 8:00 PM",
        image_url: "/assets/ham.webp",
        href: Some("https://partiful.com/e/kHobWBFIKZg7PHB8zQC6?c=At7wGl2m"),
        btn_label: "RSVP",
        rotation: "-2deg",
        pin_color: "pin-blue",
        img_position: "center 30%",
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
                                <p class="tv-flyer-label">{card.label}</p>
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
