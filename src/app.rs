use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            class="bg-blue-500 text-primary px-4 py-2 rounded"
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
