use leptos::prelude::*;
use crate::components::create::CreateHaiku;
use crate::components::view_haiku::ViewHaiku;

#[component]
pub fn HomePage() -> impl IntoView {
    let (haiku_url, set_haiku_url) = signal(Option::<String>::None);

    Effect::new(move |_| {
        if let Some(url) = haiku_url.get() {
            println!("Haiku URL: {}", url);
        }
    });

    view! {
        <div class="bg-[url('/assets/img/landing_background.png')] bg-cover bg-center bg-no-repeat h-screen">
            <Show
                when=move || haiku_url.get().is_some()
                fallback=move || view! { 
                    <CreateHaiku 
                        on_created=move |url| set_haiku_url.update(|value| *value = Some(url))
                    /> 
                }
            >
                <ViewHaiku url=haiku_url />
            </Show>
        </div>
    }
}

