use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    #[prop_or_default]
    pub eyebrow: Option<AttrValue>,
    pub title: AttrValue,
    #[prop_or_default]
    pub description: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Section)]
pub fn section(props: &SectionProps) -> Html {
    html! {
        <section class="content-section">
            <div class="shell-container">
                <div class="section-heading">
                    if let Some(eyebrow) = &props.eyebrow {
                        <p class="section-eyebrow">{eyebrow.clone()}</p>
                    }
                    <h2>{props.title.clone()}</h2>
                    if let Some(description) = &props.description {
                        <p class="section-description">{description.clone()}</p>
                    }
                </div>
                {for props.children.iter()}
            </div>
        </section>
    }
}
