// Rust WASM Web App
// #[allow(dead_code, unused_imports, unused_variables)]
mod pages;
mod components;
use pages::home::Homepage;

use components::{
    counter::Count,
    warning::UnderConstruction
};

use leptos::prelude::*;
use leptos_router::{components::*, path};

#[component]
fn App() -> impl IntoView {
    view! {
        <h2>Corrosive</h2> 
        <AppRouter/>
        <footer>
            "🔥 🔥 🔥 Written in Rust & deployed with Web Assembly 🔥 🔥 🔥"
        </footer>
    }
}

#[component]
fn AppRouter() -> impl IntoView {
    view! {
        <Router>
            <div class="nav">
                <nav>
                    <button><a href="/corrosive/">"Home"</a></button>
                    <button><a href="/corrosive/counter">"Count"</a></button>
                    <button><a href="/corrosive/about">"About"</a></button>
                </nav>
            </div>

            <div class="app-main">
                <main>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/corrosive/") view=Homepage />
                        <Route path=path!("/corrosive/counter") view=Count />
                        <Route path=path!("/corrosive/about") view=UnderConstruction />
                        <Route path=path!("/*any") view=|| view! {<h2>"This is not the way..."</h2>} />
                    </Routes>
                </main>
            </div>
        </Router>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App/>})
}
