use yew::prelude::*;

use crate::community::COMMUNITY_EVENTS_TIMELINE;

#[function_component(TvDisplay)]
pub fn tv_display() -> Html {
    let upcoming: Vec<_> = COMMUNITY_EVENTS_TIMELINE.iter().filter(|e| e.active).collect();
    let past: Vec<_> = COMMUNITY_EVENTS_TIMELINE.iter().filter(|e| !e.active).collect();

    html! {
        <div class="tv-page">
            <div class="tv-header">
                <a href="/"><img src="/assets/etc-logo-white.png" class="tv-logo" alt="Element" /></a>
                <span class="tv-heading">{"Upcoming Events"}</span>
            </div>

            <div class="tv-upcoming-grid">
                { upcoming.iter().map(|event| html! {
                    <div class="tv-event-card">
                        if let Some(img) = event.image_url {
                            <img src={img} alt={event.title} class="tv-event-card-img" />
                        }
                        <div class="tv-event-card-overlay">
                            <p class="tv-event-card-date">{event.when}</p>
                            <h2 class="tv-event-card-title">{event.title}</h2>
                            if let Some(href) = event.href {
                                <a href={href} class="tv-rsvp-btn" target="_blank">{"RSVP"}</a>
                            }
                        </div>
                    </div>
                }).collect::<Html>() }
            </div>

            if !past.is_empty() {
                <div class="tv-past-section">
                    <p class="tv-past-label">{"Past Events"}</p>
                    <div class="tv-past-scroll">
                        { past.iter().map(|event| html! {
                            <div class="tv-past-card">
                                if let Some(img) = event.image_url {
                                    <img src={img} alt={event.title} class="tv-past-card-img" />
                                }
                                <div class="tv-past-card-body">
                                    <p class="tv-past-card-date">{event.when}</p>
                                    <p class="tv-past-card-title">{event.title}</p>
                                    if let Some(href) = event.href {
                                        <a href={href} class="tv-past-card-link" target="_blank">
                                            {event.link_label.unwrap_or("View")}
                                        </a>
                                    }
                                </div>
                            </div>
                        }).collect::<Html>() }
                    </div>
                </div>
            }
        </div>
    }
}
