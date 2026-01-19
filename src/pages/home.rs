// App homepage
use leptos::prelude::*;

#[component]
pub fn Homepage() -> impl IntoView {
    view! {
        <h1>"⚠️ Corrosive ⚠️"</h1>
        <ul>
            <li>"🕸️ WASM application"</li>
            <li>"🐘 Built using Trunk WASM bundler"</li>
            <li>"🦀 Developed in Rust w/ Leptos crate"</li>
            <li>"🔧 Work in progress"</li>
        </ul>
    }
}
