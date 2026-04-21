use yew::prelude::*;

use crate::components::cta_button::CtaButton;

#[derive(Properties, PartialEq)]
pub struct PricingCardProps {
    pub title: AttrValue,
    pub price: AttrValue,
    pub cadence: AttrValue,
    pub cta_href: AttrValue,
    pub cta_label: AttrValue,
    #[prop_or_default]
    pub secondary_href: Option<AttrValue>,
    #[prop_or_default]
    pub secondary_label: Option<AttrValue>,
    #[prop_or_default]
    pub badge: Option<AttrValue>,
    #[prop_or_default]
    pub bullets: Vec<AttrValue>,
    #[prop_or(false)]
    pub featured: bool,
}

#[function_component(PricingCard)]
pub fn pricing_card(props: &PricingCardProps) -> Html {
    let classes = classes!("pricing-card", props.featured.then_some("featured"));

    html! {
        <article class={classes}>
            <div class="pricing-card-header">
                if let Some(badge) = &props.badge {
                    <span class="pricing-badge">{badge.clone()}</span>
                }
                <h3>{props.title.clone()}</h3>
                <p class="pricing-price">{props.price.clone()}</p>
                <p class="pricing-cadence">{props.cadence.clone()}</p>
            </div>

            <ul class="pricing-list">
                {props.bullets.iter().map(|bullet| html! {
                    <li>{bullet.clone()}</li>
                }).collect::<Html>()}
            </ul>

            <div class="pricing-card-actions">
                <CtaButton href={props.cta_href.clone()} label={props.cta_label.clone()} primary={props.featured} />
                if let (Some(href), Some(label)) = (&props.secondary_href, &props.secondary_label) {
                    <CtaButton href={href.clone()} label={label.clone()} primary=false />
                }
            </div>
        </article>
    }
}
