use leptos::prelude::*;
use crate::components::create::CreateHaiku;
use crate::components::view_haiku::ViewHaiku;

#[component]
pub fn HomePage() -> impl IntoView {
    let (haiku_url, set_haiku_url) = signal(Option::<String>::None);
    let (bg_class, set_bg_class) = signal("bg-cover bg-center bg-no-repeat h-screen bg-[url('/assets/img/landing_background.png')]");

    Effect::new(move |_| {
        if let Some(_) = haiku_url.get() {
            set_bg_class.set("bg-cover bg-center bg-no-repeat h-screen bg-[url('/assets/img/result_background.png')]");
        }
    });

    view! {
        <div 
            class=move || bg_class.get()
        >
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

