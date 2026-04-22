use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(ScrollToTop)]
pub fn scroll_to_top() -> Html {
    let location = use_location();

    use_effect_with(location, |_| {
        let _ = js_sys::eval("window.scrollTo(0, 0)");
    });

    html! {}
}
