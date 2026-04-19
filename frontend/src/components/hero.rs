use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeroProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub accent: Option<AttrValue>,
    pub subtitle: AttrValue,
    #[prop_or_default]
    pub kicker: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Hero)]
pub fn hero(props: &HeroProps) -> Html {
    html! {
        <section class="page-hero">
            <div class="shell-container page-hero-inner">
                <div class="page-hero-copy">
                    if let Some(kicker) = &props.kicker {
                        <p class="page-kicker">{kicker.clone()}</p>
                    }
                    <h1>
                        {props.title.clone()}
                        if let Some(accent) = &props.accent {
                            <>
                                {" "}
                                <span>{accent.clone()}</span>
                            </>
                        }
                    </h1>
                    <p class="page-subtitle">{props.subtitle.clone()}</p>
                    <div class="page-hero-actions">
                        {for props.children.iter()}
                    </div>
                </div>
            </div>
        </section>
    }
}
