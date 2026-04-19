use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[derive(Properties, PartialEq, Clone)]
pub struct BookCardProps {
    pub title: AttrValue,
    pub author: AttrValue,
    pub description: AttrValue,
    pub cover_url: AttrValue,
    pub month: AttrValue,
    #[prop_or_default]
    pub badge: Option<AttrValue>,
    #[prop_or_default]
    pub primary_href: Option<AttrValue>,
    #[prop_or_default]
    pub primary_label: Option<AttrValue>,
    #[prop_or_default]
    pub secondary_href: Option<AttrValue>,
    #[prop_or_default]
    pub secondary_label: Option<AttrValue>,
    #[prop_or_default]
    pub detail_route: Option<Route>,
    #[prop_or_default]
    pub detail_label: Option<AttrValue>,
    #[prop_or(false)]
    pub featured: bool,
}

#[function_component(BookCard)]
pub fn book_card(props: &BookCardProps) -> Html {
    let classes = classes!("community-card", props.featured.then_some("featured-card"));

    html! {
        <article class={classes}>
            <img
                class="community-card-media book-card-media"
                src={props.cover_url.clone()}
                alt={format!("{} cover", props.title)}
                loading="lazy"
            />
            <div class="community-card-body">
                if let Some(badge) = &props.badge {
                    <span class="community-badge">{badge.clone()}</span>
                }
                <p class="community-card-overline">{props.month.clone()}</p>
                <h3 class="community-card-title">{props.title.clone()}</h3>
                <p class="community-card-subtitle">{format!("by {}", props.author)}</p>
                <p class="community-card-copy">{props.description.clone()}</p>
                <div class="community-card-actions">
                    if let (Some(route), Some(label)) = (&props.detail_route, &props.detail_label) {
                        <Link<Route> to={route.clone()} classes={classes!("cta-button", "primary")}>
                            {label.clone()}
                        </Link<Route>>
                    }
                    if let (Some(href), Some(label)) = (&props.secondary_href, &props.secondary_label) {
                        <a class="cta-button" href={href.clone()} target="_blank" rel="noopener noreferrer">
                            {label.clone()}
                        </a>
                    }
                    if let (Some(href), Some(label)) = (&props.primary_href, &props.primary_label) {
                        <a class="cta-button primary" href={href.clone()} target="_blank" rel="noopener noreferrer">
                            {label.clone()}
                        </a>
                    }
                </div>
            </div>
        </article>
    }
}
