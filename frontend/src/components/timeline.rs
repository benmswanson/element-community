use yew::prelude::*;

use crate::community::TimelineEntry;

#[derive(Properties, PartialEq)]
pub struct TimelineProps {
    pub items: Vec<TimelineEntry>,
}

#[function_component(Timeline)]
pub fn timeline(props: &TimelineProps) -> Html {
    html! {
        <div class="timeline">
            {props.items.iter().map(|item| html! {
                <article class={classes!("timeline-item", item.active.then_some("active"))}>
                    if let Some(url) = item.image_url {
                        <img class="timeline-image" src={url} alt={item.title} loading="lazy" />
                    }
                    <p class="timeline-when">{item.when}</p>
                    <h3 class="timeline-title">{item.title}</h3>
                    <p class="timeline-detail">{item.detail}</p>
                    if let (Some(href), Some(label)) = (item.href, item.link_label) {
                        <a class="timeline-link" href={href} target="_blank" rel="noopener noreferrer">
                            {label}
                        </a>
                    }
                </article>
            }).collect::<Html>()}
        </div>
    }
}
