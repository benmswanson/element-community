// frontend/src/components/event_card.rs
//
// Featured event card matching the BookCard layout but for runs and events.

use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct EventCardProps {
    pub title: String,
    pub date: String,
    pub description: String,
    pub icon: String,
    #[prop_or_default]
    pub location: Option<String>,
    #[prop_or_default]
    pub image_url: Option<String>,
    #[prop_or_default]
    pub rsvp_url: Option<String>,
    #[prop_or_default]
    pub secondary_url: Option<String>,
    #[prop_or_default]
    pub secondary_label: Option<String>,
    #[prop_or_default]
    pub badge_label: Option<String>,
    #[prop_or_default]
    pub badge_icon: Option<String>,
}

#[function_component(EventCard)]
pub fn event_card(props: &EventCardProps) -> Html {
    html! {
        <div class="material-card featured-card">
            if let Some(url) = &props.image_url {
                <img class="card-media" src={url.clone()} alt={props.title.clone()} loading="lazy" />
            } else {
                <div class="card-media event-card-media">
                    <span class="event-card-icon material-symbols-outlined">{&props.icon}</span>
                </div>
            }
            <div class="card-body">
                if let (Some(label), Some(icon)) = (&props.badge_label, &props.badge_icon) {
                    <span class="badge badge-active">
                        <span class="material-symbols-outlined">{icon.clone()}</span>
                        {label.clone()}
                    </span>
                }
                <div class="card-overline">{&props.date}</div>
                <div class="card-title">{&props.title}</div>
                if let Some(loc) = &props.location {
                    <div class="card-location">
                        <span class="material-symbols-outlined" style="font-size:0.95rem;vertical-align:middle;margin-right:0.25rem;">{"location_on"}</span>
                        {loc.clone()}
                    </div>
                }
                <p class="card-text">{&props.description}</p>
                <div class="card-actions">
                    if let (Some(url), Some(label)) = (&props.secondary_url, &props.secondary_label) {
                        <a class="btn btn-outline" href={url.clone()} target="_blank" rel="noopener">
                            {label.clone()}
                            <span class="material-symbols-outlined">{"open_in_new"}</span>
                        </a>
                    }
                    if let Some(url) = &props.rsvp_url {
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
