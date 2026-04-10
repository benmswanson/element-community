// frontend/src/app.rs

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{footer::Footer, nav::Nav};
use crate::pages::{home::Home, schedule::Schedule, about::About};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/schedule")]
    Schedule,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Schedule => html! { <Schedule /> },
        Route::About => html! { <About /> },
        Route::NotFound => html! { <Redirect<Route> to={Route::Home} /> },
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
