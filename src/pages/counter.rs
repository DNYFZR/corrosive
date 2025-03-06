use leptos::prelude::*;

#[component]
pub fn Count() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <div>
            <button
                on:click=move |_| {
                    *set_count.write() += 1;
                }
            >
                "Click Me"
            </button>
            
            <div>
                // Button output
                {move || if count.get() == 0 {
                    "".to_string()
                } else {
                    count.get().to_string()
                }}
            </div>
        </div>
    }
}
