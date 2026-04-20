use yew::prelude::*;

use crate::components::{section::Section, team_card::TeamCard};

struct Coach {
    name: &'static str,
    role: &'static str,
    bio: &'static str,
}

const COACHES: [Coach; 6] = [
    Coach {
        name: "Blake Guillaume",
        role: "General Manager | Group Coach",
        bio: "Keeps the experience grounded, welcoming, and community-first while helping members find the right way into training.",
    },
    Coach {
        name: "Sierra Baker",
        role: "Head Coach | Group Coach",
        bio: "Leads programming with a balanced approach to strength, conditioning, and mobility so classes stay challenging, clear, and approachable.",
    },
    Coach {
        name: "Daniel Garcia",
        role: "Group Coach",
        bio: "Brings steady energy to the floor and helps members feel coached through each session rather than left to figure it out alone.",
    },
    Coach {
        name: "Miguel Sira",
        role: "Group Coach",
        bio: "Supports strong movement quality and consistency with a coaching style that is clear, encouraging, and athlete-minded.",
    },
    Coach {
        name: "Simba Wedderburn",
        role: "Group Coach",
        bio: "Creates a welcoming training atmosphere where members can push hard, learn the work, and enjoy doing it together.",
    },
    Coach {
        name: "Maria Sui",
        role: "Group Coach",
        bio: "Helps members build confidence in class through guidance that is attentive, upbeat, and rooted in progress over perfection.",
    },
];

#[function_component(Team)]
pub fn team() -> Html {
    html! {
        <>
            <Section
                eyebrow="Our Team"
                title="Meet the people who set the tone."
                description="The coaching staff brings the programming to life and makes the room feel motivating, inclusive, and consistent."
            >
                <div class="team-grid">
                    {COACHES.iter().map(|coach| html! {
                        <TeamCard
                            name={coach.name}
                            role={coach.role}
                            bio={Some(AttrValue::from(coach.bio))}
                        />
                    }).collect::<Html>()}
                </div>
            </Section>
        </>
    }
}
