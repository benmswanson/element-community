use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

struct Coach {
    first_name: &'static str,
    full_name: &'static str,
    role: &'static str,
    photo_url: &'static str,
    route: Route,
}

const COACHES: [Coach; 4] = [
    Coach {
        first_name: "Blake",
        full_name: "Blake Guillaume",
        role: "General Manager | Group Coach",
        photo_url: "https://images.squarespace-cdn.com/content/v1/67c33d5dc8308d77d48dcc4c/cc3d75ca-38a3-4a6a-bae8-5e3980b56302/select-5.jpg",
        route: Route::TeamBlake,
    },
    Coach {
        first_name: "Sierra",
        full_name: "Sierra Baker",
        role: "Head Coach | Group Coach",
        photo_url: "https://images.squarespace-cdn.com/content/v1/67c33d5dc8308d77d48dcc4c/daafa140-2927-4d82-9a09-52724bc127fe/select-2.jpg",
        route: Route::TeamSierra,
    },
    Coach {
        first_name: "Miguel",
        full_name: "Miguel Sira",
        role: "Group Coach",
        photo_url: "https://images.squarespace-cdn.com/content/v1/67c33d5dc8308d77d48dcc4c/cf7fd3d8-3908-4853-9856-689c7d294d8e/select-15.jpg",
        route: Route::TeamMiguel,
    },
    Coach {
        first_name: "Simba",
        full_name: "Simba Wedderburn",
        role: "Group Coach",
        photo_url: "https://images.squarespace-cdn.com/content/v1/67c33d5dc8308d77d48dcc4c/1d2a7cfb-d4b6-4cf2-81e0-10d1e0ff2e25/select-3.jpg",
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
                            <img src={coach.photo_url} alt={coach.full_name} />
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
