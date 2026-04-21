use yew::prelude::*;

#[function_component(Schedule)]
pub fn schedule() -> Html {
    html! {
        <div class="shell-container schedule-embed">
            <div class="mindbody-widget" data-widget-type="Schedules" data-widget-id="ea342103127">
                <iframe
                    id="bw-widget-iframe-f1007"
                    class="bw-widget-styles"
                    src="https://go.mindbodyonline.com/book/widgets/schedules/view/ea342103127/schedule"
                    allow="payment"
                    scrolling="no"
                    style="overflow: hidden; width: 100%; height: 896px; border: none;"
                />
            </div>
        </div>
    }
}
