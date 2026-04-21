use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{
    book_club::BookClub, classes::Classes, events::Events, home::Home,
    pricing::Pricing, run_club::RunClub, schedule::Schedule, team::Team,
    team_bio::{BioBlake, BioSierra, BioDaniel, BioMiguel, BioSimba, BioMaria},
};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/team")]
    Team,
    #[at("/team/blake")]
    TeamBlake,
    #[at("/team/sierra")]
    TeamSierra,
    #[at("/team/daniel")]
    TeamDaniel,
    #[at("/team/miguel")]
    TeamMiguel,
    #[at("/team/simba")]
    TeamSimba,
    #[at("/team/maria")]
    TeamMaria,
    #[at("/schedule")]
    Schedule,
    #[at("/pricing")]
    Pricing,
    #[at("/classes")]
    Classes,
    #[at("/book-club")]
    BookClub,
    #[at("/run-club")]
    RunClub,
    #[at("/community-events")]
    Events,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Team => html! { <Team /> },
        Route::TeamBlake => html! { <BioBlake /> },
        Route::TeamSierra => html! { <BioSierra /> },
        Route::TeamDaniel => html! { <BioDaniel /> },
        Route::TeamMiguel => html! { <BioMiguel /> },
        Route::TeamSimba => html! { <BioSimba /> },
        Route::TeamMaria => html! { <BioMaria /> },
        Route::Schedule => html! { <Schedule /> },
        Route::Pricing => html! { <Pricing /> },
        Route::Classes => html! { <Classes /> },
        Route::BookClub => html! { <BookClub /> },
        Route::RunClub => html! { <RunClub /> },
        Route::Events => html! { <Events /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Home} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="site-shell">
                <Nav />
                <main class="site-main">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}
