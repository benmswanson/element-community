use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    let current = use_route::<Route>().unwrap_or(Route::Home);
    let menu_open = use_state(|| false);

    let is_active = |route: &Route| current == *route;

    let toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_| menu_open.set(!*menu_open))
    };

    let close_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| menu_open.set(false))
    };

    let nav_link = |route: Route, label: &'static str| {
        let classes = classes!(
            "site-nav-link",
            is_active(&route).then_some("active"),
        );
        html! {
            <Link<Route> to={route} classes={classes}>
                {label}
            </Link<Route>>
        }
    };

    html! {
        <header class="site-header">
            <div class="shell-container site-header-inner">
                <Link<Route> to={Route::Home} classes="site-brand">
                    <img src="/assets/etc-logo-white.png" alt="Element Training Club" class="site-brand-mark" />
                </Link<Route>>

                <button
                    class={classes!("site-menu-toggle", (*menu_open).then_some("open"))}
                    type="button"
                    aria-label="Toggle menu"
                    aria-expanded={(*menu_open).to_string()}
                    onclick={toggle_menu}
                >
                    <span></span>
                    <span></span>
                    <span></span>
                </button>

                <nav class={classes!("site-nav", (*menu_open).then_some("open"))} aria-label="Primary" onclick={close_menu.clone()}>
                    {nav_link(Route::Home, "Home")}
                    {nav_link(Route::Team, "Team")}
                    {nav_link(Route::Schedule, "Schedule")}
                    {nav_link(Route::Pricing, "Pricing")}
                    {nav_link(Route::Classes, "Classes")}
                    {nav_link(Route::BookClub, "Book Club")}
                    {nav_link(Route::RunClub, "Run Club")}
                    {nav_link(Route::Events, "Community Events")}
                    <a
                        class="site-nav-cta"
                        href="https://clients.mindbodyonline.com/classic/ws?studioid=5735683&stype=-7&sTG=23&sVT=517&sView=day&sLoc=0"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {"Book a Class"}
                    </a>
                </nav>
            </div>
        </header>
    }
}
