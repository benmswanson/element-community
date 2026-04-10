// frontend/src/components/footer.rs

use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="footer">
            <div class="footer-inner">
                <span>{"© 2026 Element Book Club"}</span>
                <span>{"Built with 🦀 Rust + Yew"}</span>
            </div>
        </footer>
    }
}
