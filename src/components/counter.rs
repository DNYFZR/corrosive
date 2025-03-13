use leptos::prelude::*;

#[component]
pub fn Count() -> impl IntoView {
    // create a list of independent signals
    let counters = vec![0; 20].into_iter().map(|idx| RwSignal::new(idx));
    
    // Map to individual buttons
    let counter_buttons = counters
        .enumerate()
        .map(|(n, count)| {
            let id = format!("counter {} :", n+1);
            view! {
                <p>{id} <button on:click=move |_| *count.write() += 1>{count}</button></p>
            }
        })
        .collect_view();

    // Render button view
    view! {
        <ul>{counter_buttons}</ul>
    }
}
