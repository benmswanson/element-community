use yew::prelude::*;

use crate::components::section::Section;

#[function_component(Schedule)]
pub fn schedule() -> Html {
    html! {
        <>
            <Section
                eyebrow="Coming Soon"
                title="Calendar coming soon."
                description="We're building an integrated calendar for class schedules and community events. Check back soon."
            >
            </Section>
        </>
    }
}
