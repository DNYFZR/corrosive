// Rust WASM Web App
#[allow(dead_code, unused_imports, unused_variables)]
#[allow(non_snake_case)]
use leptos::prelude::*;
use leptos_router::{components::*, path};

mod components;
mod pages;

use crate::{
    components::warnings::{NotFound, UnderConstruction},
    pages::home::Homepage,
};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<App />})
}

#[component]
fn App() -> impl IntoView {
    let base_url = "corrosive";
    let (menu, show_menu) = signal(false);

    view! {
        <div>
            // App menu
            <button
                class="menu-button-container"
                on:click = move |_| *show_menu.write() = !menu.get()
            >
                <img
                    class="menu-button"
                    src="./public/menu-128.png"
                    alt="menu button"
                />
            </button>

            // Routing Menu
            <Router>
                <Show when=move || menu.get() == true>
                    <nav class="nav">
                        <button class="menu-item">
                            <a href=format!("/{base_url}/")>"Home"</a>
                        </button>
                        <button class="menu-item">
                            <a href=format!("/{base_url}/help")>"Help"</a>
                        </button>
                    </nav>
                </Show>

                <main class="pages">
                    <Routes fallback = || NotFound>
                        <Route path=path!("/corrosive/") view=Homepage />
                        <Route path=path!("/corrosive/help") view=UnderConstruction />
                    </Routes>
                </main>
            </Router>
        </div>
    }
}
