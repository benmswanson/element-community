use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CtaButtonProps {
    pub href: AttrValue,
    pub label: AttrValue,
    #[prop_or(false)]
    pub primary: bool,
}

#[function_component(CtaButton)]
pub fn cta_button(props: &CtaButtonProps) -> Html {
    let classes = classes!("cta-button", props.primary.then_some("primary"));

    html! {
        <a
            class={classes}
            href={props.href.clone()}
            target="_blank"
            rel="noopener noreferrer"
        >
            {props.label.clone()}
        </a>
    }
}
