use leptos::prelude::*;


#[component]
pub fn ViewHaiku(url: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="h-screen mx-auto max-w-6xl flex items-center justify-center">
            <img src={move || url.get().unwrap()} class="w-3/4" />
        </div>
    }
}
