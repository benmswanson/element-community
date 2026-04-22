use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[derive(PartialEq)]
pub struct BioData {
    pub full_name: &'static str,
    pub role: &'static str,
    pub photo_url: &'static str,
    pub paragraphs: &'static [&'static str],
}

pub const BLAKE: BioData = BioData {
    full_name: "Blake Guillaume",
    role: "General Manager | Group Coach",
    photo_url: "/assets/select-5.jpg",
    paragraphs: &[
        "Blake is a former Bain consultant who transitioned from corporate consulting to pursue his passion for building community through fitness.",
        "As a lifelong athlete, he founded Element Training Club in Williamsburg with a commitment to inclusive, community-centered fitness. Blake's philosophy centers on three core principles: physical gratitude, balanced training, and self-challenge.",
        "He encourages members to view training as a lifelong journey that supports mental health, confidence, and overall wellness — not merely aesthetic goals. His athletic background spans multiple disciplines including sports, running, and strength training, all of which taught him the value of community through movement.",
        "He now channels that energy into Element, creating a welcoming space where people of all backgrounds can train seriously, support one another, and grow together.",
    ],
};

pub const SIERRA: BioData = BioData {
    full_name: "Sierra Baker",
    role: "Head Coach | Programming Lead",
    photo_url: "/assets/select-2.jpg",
    paragraphs: &[
        "Sierra was raised in Washington, D.C. and began competitive swimming at age five. She competed at Division I level for four years at Rice University while earning a degree in Mechanical Engineering.",
        "Her athletic accomplishments include being a two-time HYROX champion, HYROX World Championships competitor, 70.3 Ironman Triathlon racer who qualified for World Championships in Nice, France, and amateur boxing competitor.",
        "With over six years in the fitness industry coaching athletes across various sports, training disciplines, and experience levels, her coaching philosophy is: \"When intention meets accountability, progress is inevitable.\"",
        "Her approach blends community, empowerment, and discipline to foster a team-like culture in a high-energy environment.",
    ],
};

pub const DANIEL: BioData = BioData {
    full_name: "Daniel Garcia",
    role: "Group Coach",
    photo_url: "/assets/select-15.jpg",
    paragraphs: &[
        "Daniel started his fitness journey in 2017 through CrossFit, initially pursuing weight loss but eventually committing to lifelong movement and strength training.",
        "He competes in endurance races and HYROX events while maintaining a consistent training routine. Daniel is passionate about weightlifting, particularly Olympic lifting, with the Clean and Jerk as his favorite lift.",
        "His current training emphasizes cardio through HIIT classes and spin. He believes fitness is accessible to everyone regardless of appearance or performance level and encourages people to \"start where you are and build from there.\"",
        "Daniel emphasizes the importance of community: \"You can't be what you can't see, and that's why training in a supportive community matters.\"",
    ],
};

pub const MIGUEL: BioData = BioData {
    full_name: "Miguel Sira",
    role: "Group Coach",
    photo_url: "/assets/select-15.jpg",
    paragraphs: &[
        "Miguel brings an athlete-minded coaching style to every session, focused on strong movement quality and helping members build consistency over time.",
    ],
};

pub const SIMBA: BioData = BioData {
    full_name: "Simba Wedderburn",
    role: "Group Coach",
    photo_url: "/assets/select-3.jpg",
    paragraphs: &[
        "Simba focuses on functional strength and athletic performance with an emphasis on moving well, building resilience, and developing real-world strength.",
        "His training integrates mobility, stability, core control, and joint-friendly methods alongside strength endurance and body recomposition to create balanced, capable bodies.",
        "As an artist, Simba sees movement as a holistic bridge between the body and the inner self — a way to align with the part of us that feels, creates, and experiences life fully. When the body moves with intention, the mind follows with clarity and presence.",
        "His philosophy is simple: train smart, move with purpose, and build a body that supports your lifestyle, creativity, and long-term performance.",
    ],
};

pub const MARIA: BioData = BioData {
    full_name: "Maria Sui",
    role: "Group Coach",
    photo_url: "https://images.squarespace-cdn.com/content/v1/67c33d5dc8308d77d48dcc4c/6123cb34-d336-458c-80cd-623958d90d33/IMG_4499.jpg",
    paragraphs: &[
        "Maria helps members build confidence in class through guidance that is attentive, upbeat, and rooted in progress over perfection.",
    ],
};

#[derive(Properties, PartialEq)]
pub struct TeamBioProps {
    pub data: &'static BioData,
}

#[function_component(TeamBio)]
pub fn team_bio(props: &TeamBioProps) -> Html {
    let data = props.data;
    html! {
        <div class="shell-container bio-page">
            <Link<Route> to={Route::Team} classes="bio-back">{"← Back to Team"}</Link<Route>>
            <div class="bio-layout">
                <div class="bio-photo-col">
                    <img src={data.photo_url} alt={data.full_name} class="bio-photo" />
                </div>
                <div class="bio-content-col">
                    <p class="card-label">{data.role}</p>
                    <h1 class="bio-name">{data.full_name}</h1>
                    {data.paragraphs.iter().map(|p| html! {
                        <p class="bio-paragraph">{*p}</p>
                    }).collect::<Html>()}
                </div>
            </div>
        </div>
    }
}

#[function_component(BioBlake)]
pub fn bio_blake() -> Html { html! { <TeamBio data={&BLAKE} /> } }
#[function_component(BioSierra)]
pub fn bio_sierra() -> Html { html! { <TeamBio data={&SIERRA} /> } }
#[function_component(BioDaniel)]
pub fn bio_daniel() -> Html { html! { <TeamBio data={&DANIEL} /> } }
#[function_component(BioMiguel)]
pub fn bio_miguel() -> Html { html! { <TeamBio data={&MIGUEL} /> } }
#[function_component(BioSimba)]
pub fn bio_simba() -> Html { html! { <TeamBio data={&SIMBA} /> } }
#[function_component(BioMaria)]
pub fn bio_maria() -> Html { html! { <TeamBio data={&MARIA} /> } }
