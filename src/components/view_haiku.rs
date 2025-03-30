use leptos::prelude::*;
use leptos::task::spawn_local;


#[component]
pub fn ViewHaiku(url: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div class="h-screen mx-auto max-w-6xl flex items-center">
            <img src={move || url.get().unwrap()} class="w-2/3" />
        </div>
    }
}
