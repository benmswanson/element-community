// frontend/src/components/nav.rs

use crate::app::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let current = use_route::<Route>().unwrap_or(Route::Home);

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
                        <Link<Route> to={Route::Home} classes={active(&Route::Home)}>
                            {"Home"}
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::Schedule} classes={active(&Route::Schedule)}>
                            {"Schedule"}
                        </Link<Route>>
                    </li>
                    <li>
                        <Link<Route> to={Route::About} classes={active(&Route::About)}>
                            {"About"}
                        </Link<Route>>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
