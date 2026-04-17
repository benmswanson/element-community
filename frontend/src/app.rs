// frontend/src/app.rs

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{book_club::BookClub, run_club::RunClub, events::Events};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    BookClub,
    #[at("/run-club")]
    RunClub,
    #[at("/events")]
    Events,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::BookClub => html! { <BookClub /> },
        Route::RunClub => html! { <RunClub /> },
        Route::Events => html! { <Events /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::BookClub} /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="site-wrapper">
                <Nav />
                <main class="main-content">
                    <div class="container">
                        <Switch<Route> render={switch} />
                    </div>
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}
