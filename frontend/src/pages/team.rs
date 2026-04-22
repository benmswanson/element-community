use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

struct Coach {
    first_name: &'static str,
    full_name: &'static str,
    role: &'static str,
    photo_url: &'static str,
    photo_class: &'static str,
    route: Route,
}

const COACHES: [Coach; 4] = [
    Coach {
        first_name: "Blake",
        full_name: "Blake Guillaume",
        role: "General Manager | Group Coach",
        photo_url: "/assets/select-5.jpg",
        photo_class: "photo-zoom-in",
        route: Route::TeamBlake,
    },
    Coach {
        first_name: "Sierra",
        full_name: "Sierra Baker",
        role: "Head Coach | Group Coach",
        photo_url: "/assets/select-2.jpg",
        photo_class: "",
        route: Route::TeamSierra,
    },
    Coach {
        first_name: "Miguel",
        full_name: "Miguel Sira",
        role: "Group Coach",
        photo_url: "/assets/select-15.jpg",
        photo_class: "photo-zoom-in photo-position-high",
        route: Route::TeamMiguel,
    },
    Coach {
        first_name: "Simba",
        full_name: "Simba Wedderburn",
        role: "Group Coach",
        photo_url: "/assets/select-3.jpg",
        photo_class: "photo-zoom-more",
        route: Route::TeamSimba,
    },
];

#[function_component(Team)]
pub fn team() -> Html {
    html! {
        <div class="shell-container team-page">
            <div class="team-cards-grid">
                {COACHES.iter().map(|coach| html! {
                    <Link<Route> to={coach.route.clone()} classes="team-photo-card">
                        <div class="team-photo-wrap">
                            <img src={coach.photo_url} alt={coach.full_name} class={coach.photo_class} />
                        </div>
                        <div class="team-photo-card-body">
                            <p class="card-label">{coach.role}</p>
                            <h3>{coach.full_name}</h3>
                            <p class="team-card-link-hint">{format!("About {}", coach.first_name)}</p>
                        </div>
                    </Link<Route>>
                }).collect::<Html>()}
            </div>
        </div>
    }
}
