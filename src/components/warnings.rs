// Error Handling Routes
use leptos::{IntoView, component, html::*, view};

#[component]
pub fn UnderConstruction() -> impl IntoView {
    view! {
        <h2>"🚧 Page Under Construction 🚧"</h2>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <h2>"⛔ 404 Page Not Found ⛔"</h2>
    }
}
