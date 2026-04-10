// frontend/src/components/book_card.rs
//
// Material-style card for displaying a book pick. Supports both
// a compact grid card and a featured (horizontal) layout.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BookCardProps {
    pub title: String,
    pub author: String,
    pub description: String,
    pub cover_url: String,
    pub month: String,
    #[prop_or_default]
    pub goodreads_url: Option<String>,
    #[prop_or_default]
    pub partiful_url: Option<String>,
    #[prop_or_default]
    pub is_current: bool,
    #[prop_or_default]
    pub featured: bool,
}

#[function_component(BookCard)]
pub fn book_card(props: &BookCardProps) -> Html {
    let card_class = if props.featured {
        "material-card featured-card"
    } else {
        "material-card"
    };

    html! {
        <div class={card_class}>
            <img
                class="card-media"
                src={props.cover_url.clone()}
                alt={format!("{} cover", props.title)}
                loading="lazy"
            />
            <div class="card-body">
                if props.is_current {
                    <span class="badge badge-active">
                        <span class="material-symbols-outlined">{"menu_book"}</span>
                        {"Reading Now"}
                    </span>
                }
                <div class="card-overline">{&props.month}</div>
                <div class="card-title">{&props.title}</div>
                <div class="card-subtitle">{format!("by {}", &props.author)}</div>
                <p class="card-text">{&props.description}</p>
                <div class="card-actions">
                    if let Some(url) = &props.goodreads_url {
                        <a class="btn btn-outline" href={url.clone()} target="_blank" rel="noopener">
                            {"Goodreads"}
                            <span class="material-symbols-outlined">{"open_in_new"}</span>
                        </a>
                    }
                    if let Some(url) = &props.partiful_url {
                        <a class="btn btn-primary" href={url.clone()} target="_blank" rel="noopener">
                            <span class="material-symbols-outlined">{"event"}</span>
                            {"RSVP"}
                        </a>
                    }
                </div>
            </div>
        </div>
    }
}
