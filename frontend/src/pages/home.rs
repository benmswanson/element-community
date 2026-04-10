// frontend/src/pages/home.rs
//
// Landing page: hero section, current read (featured card),
// how-it-works info cards, and quick links.

use crate::app::Route;
use crate::components::book_card::BookCard;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <>
            // --- Hero ---
            <section class="hero">
                <h1>{"Welcome to "}<span class="hero-accent">{"Element Book Club"}</span></h1>
                <p class="hero-subtitle">
                    {"The only official"}
                    <span class="hero-asterisk">{"*"}</span>
                    {" book club of Element Training Club"}
                </p>
                <p class="hero-fine-print">
                    {"*all others who try to claim official will be pursued to the full extent of the law by Element Legal Club"}
                </p>
            </section>

            // --- Current Read ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// currently reading"}</span>
                    <Link<Route> to={Route::Schedule} classes="section-link">
                        {"Full schedule →"}
                    </Link<Route>>
                </div>

                <BookCard
                    title="Headshot"
                    author="Rita Bullwinkel"
                    description="An electrifying novel about a women's boxing tournament in Reno, Nevada, featuring eight teenage competitors. The narrative mirrors the tournament bracket, weaving together each fighter's past, present, and future while exploring themes of ambition, physicality, and the fleeting nature of pivotal moments."
                    cover_url="https://m.media-amazon.com/images/S/compressed.photo.goodreads.com/books/1691077222i/174156218.jpg"
                    month="April 2026"
                    goodreads_url="https://www.goodreads.com/book/show/174156218"
                    partiful_url="https://partiful.com/e/Z0UI3U0H7Ooe1XgYdfHr?c=AfUZfvML"
                    is_current=true
                    featured=true
                />
            </section>

            // --- How It Works ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// how it works"}</span>
                </div>

                <div class="info-grid">
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"group_add"}</span>
                        <h3>{"Join the Group"}</h3>
                        <p>{"Hop into our Goodreads group to see picks, chat, and track your reading."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"auto_stories"}</span>
                        <h3>{"Read the Book"}</h3>
                        <p>{"We announce the pick at the start of each month. Read at your own pace."}</p>
                    </div>
                    <div class="info-card">
                        <span class="info-icon material-symbols-outlined">{"forum"}</span>
                        <h3>{"Meet & Discuss"}</h3>
                        <p>{"RSVP to the meetup and come share your thoughts — spoilers welcome."}</p>
                    </div>
                </div>
            </section>

            // --- Quick Links ---
            <section class="section">
                <div class="section-header">
                    <span class="section-title">{"// get involved"}</span>
                </div>
                <div class="card-actions" style="border-top: none; padding-top: 0;">
                    <a class="btn btn-primary" href="https://www.goodreads.com/group/invite/7267353-element-book-club?invite_token=MmNiYmVkZTYtMWM5ZC00MzMyLThhMWEtMDMxMDUyNTliZDNi&utm_medium=email&utm_source=copypastegroup" target="_blank" rel="noopener">
                        <span class="material-symbols-outlined">{"group"}</span>
                        {"Join Goodreads Group"}
                    </a>
                    <a class="btn btn-outline" href="https://forms.gle/eQFo1SqXJwRfr3tX6" target="_blank" rel="noopener">
                        <span class="material-symbols-outlined">{"lightbulb"}</span>
                        {"Suggest a Book"}
                    </a>
                </div>
            </section>
        </>
    }
}
