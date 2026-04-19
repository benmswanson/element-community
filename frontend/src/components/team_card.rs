use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TeamCardProps {
    pub name: AttrValue,
    pub role: AttrValue,
    #[prop_or_default]
    pub bio: Option<AttrValue>,
}

#[function_component(TeamCard)]
pub fn team_card(props: &TeamCardProps) -> Html {
    html! {
        <article class="info-panel team-card">
            <p class="card-label">{"Coach"}</p>
            <h3>{props.name.clone()}</h3>
            <p class="card-meta">{props.role.clone()}</p>
            if let Some(bio) = &props.bio {
                <p class="card-copy">{bio.clone()}</p>
            }
        </article>
    }
}
