// frontend/src/components/nav.rs

use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let current = use_route::<Route>().unwrap_or(Route::BookClub);

    let active = |route: &Route| -> &'static str {
        if &current == route {
            "nav-link active"
        } else {
            "nav-link"
        }
    };

    html! {
        <nav class="nav">
            <div class="nav-inner">
                <a href="https://www.element-training.com/" target="_blank" rel="noopener" class="nav-logo">
                    <div class="nav-logo-element-wrap">
                        <img src="/assets/etc-logo-white.png" alt="Element Training Club" class="nav-logo-element" />
                    </div>
                </a>
                <ul class="nav-links">
                    <li>
                        <Link<Route> to={Route::BookClub} classes={active(&Route::BookClub)}>
                            {"Book Club"}
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::RunClub} classes={active(&Route::RunClub)}>
                            {"Run Club"}
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Events} classes={active(&Route::Events)}>
                            {"Community Events"}
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
