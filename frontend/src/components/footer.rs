use yew::prelude::*;

struct FooterLink {
    label: &'static str,
    href: &'static str,
}

const SUPPORT_LINKS: [FooterLink; 3] = [
    FooterLink {
        label: "Member Policies",
        href: "https://www.element-training.com/member-policies",
    },
    FooterLink {
        label: "Privacy Policy",
        href: "https://www.element-training.com/private-policy",
    },
    FooterLink {
        label: "FAQs",
        href: "https://www.element-training.com/faqs",
    },
];

const SOCIAL_LINKS: [FooterLink; 2] = [
    FooterLink {
        label: "Instagram",
        href: "https://www.instagram.com/elementtrainingclub/",
    },
    FooterLink {
        label: "Spotify",
        href: "https://open.spotify.com/",
    },
];

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="site-footer">
            <div class="shell-container site-footer-grid">
                <div class="site-footer-block">
                    <p class="footer-eyebrow">{"Element Training Club"}</p>
                    <p>
                        {"Coach-led strength, conditioning, and mobility training in Williamsburg, Brooklyn."}
                    </p>
                </div>

                <div class="site-footer-block">
                    <p class="footer-eyebrow">{"Visit"}</p>
                    <p>{"667 Grand Street"}</p>
                    <p>{"Brooklyn, NY 11211"}</p>
                </div>

                <div class="site-footer-block">
                    <p class="footer-eyebrow">{"Contact"}</p>
                    <a href="mailto:blake@element-training.com">{"blake@element-training.com"}</a>
                    <a href="tel:+19547785650">{"(954) 778-5650"}</a>
                </div>

                <div class="site-footer-block">
                    <p class="footer-eyebrow">{"Support"}</p>
                    {
                        SUPPORT_LINKS.iter().map(|link| html! {
                            <a href={link.href} target="_blank" rel="noopener noreferrer">{link.label}</a>
                        }).collect::<Html>()
                    }
                </div>

                <div class="site-footer-block">
                    <p class="footer-eyebrow">{"Follow"}</p>
                    {
                        SOCIAL_LINKS.iter().map(|link| html! {
                            <a href={link.href} target="_blank" rel="noopener noreferrer">{link.label}</a>
                        }).collect::<Html>()
                    }
                </div>
            </div>

            <div class="shell-container site-footer-bottom">
                <span>{"© 2026 Element Training Club"}</span>
                <span>{"Built in Rust + Yew"}</span>
            </div>
        </footer>
    }
}
