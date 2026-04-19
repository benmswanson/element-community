use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ClassCardProps {
    pub title: AttrValue,
    pub description: AttrValue,
    #[prop_or_default]
    pub bullets: Vec<AttrValue>,
}

#[function_component(ClassCard)]
pub fn class_card(props: &ClassCardProps) -> Html {
    html! {
        <article class="info-panel class-card">
            <p class="card-label">{"Class"}</p>
            <h3>{props.title.clone()}</h3>
            <p class="card-copy">{props.description.clone()}</p>
            <ul class="detail-list">
                {props.bullets.iter().map(|bullet| html! { <li>{bullet.clone()}</li> }).collect::<Html>()}
            </ul>
        </article>
    }
}
