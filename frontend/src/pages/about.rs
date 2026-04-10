// frontend/src/pages/about.rs

use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="about-page">
            <h1>{"About Element Book Club"}</h1>
            <p>
                {"Element Book Club is a monthly reading group for people who want to read more intentionally. Each month we pick a book together, read it on our own time, and then meet up to talk about it."}
            </p>
            <p>
                {"No pressure, no quizzes, no homework — just good books and good conversation. Whether you finish the whole thing or only get through a few chapters, you're welcome at the table."}
            </p>
            <p>
                {"We coordinate through our Goodreads group and host meetups via Partiful. Want to suggest the next read? We've got a form for that."}
            </p>

            <div class="section" style="margin-top: 2.5rem;">
                <div class="section-header">
                    <span class="section-title">{"// links"}</span>
                </div>
                <div class="info-grid">
                    <a class="info-card" href="https://www.goodreads.com/group/invite/7267353-element-book-club?invite_token=MmNiYmVkZTYtMWM5ZC00MzMyLThhMWEtMDMxMDUyNTliZDNi&utm_medium=email&utm_source=copypastegroup" target="_blank" rel="noopener" style="text-decoration:none;color:inherit;">
                        <span class="info-icon material-symbols-outlined">{"group"}</span>
                        <h3>{"Goodreads Group"}</h3>
                        <p>{"Join the group, see what we're reading, and leave reviews."}</p>
                    </a>
                    <a class="info-card" href="https://forms.gle/eQFo1SqXJwRfr3tX6" target="_blank" rel="noopener" style="text-decoration:none;color:inherit;">
                        <span class="info-icon material-symbols-outlined">{"lightbulb"}</span>
                        <h3>{"Suggest a Book"}</h3>
                        <p>{"Got a book you think we should read? Drop it in the form."}</p>
                    </a>
                    <a class="info-card" href="https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML" target="_blank" rel="noopener" style="text-decoration:none;color:inherit;">
                        <span class="info-icon material-symbols-outlined">{"event"}</span>
                        <h3>{"April Meetup"}</h3>
                        <p>{"RSVP for our discussion of Headshot by Rita Bullwinkel."}</p>
                    </a>
                </div>
            </div>
        </div>
    }
}
