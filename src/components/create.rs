use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::services::gemini_image_gen::ImageGenerator;

#[server]
pub async fn generate_haiku(
    line_one: String, 
    line_two: String, 
    line_three: String,
) -> Result<String, ServerFnError> {
    println!("{}-{}-{}", line_one, line_two, line_three);
    let prompt = format!("A serene ink wash painting in the style of traditional Japanese sumi-e, capturing the essence of the following haiku:
            Line one: {}
            Line two: {}
            Line three: {}
        Painted on a lightly textured rice paper background with delicate brushstrokes and minimal color. The atmosphere is tranquil and evokes the feeling of timeless Japanese aesthetics. Subtle shadows and soft gradients of black ink bring depth, while any color is kept understated—focusing on simplicity, elegance, and the fleeting beauty at the heart of the poem.",
        line_one, line_two, line_three
    );

    let generator = ImageGenerator::new("imagen-3.0-generate-002", &prompt)?;

    let generated = generator.generate().await?;
    let data_url = generated.to_data_url();

    Ok(data_url)
}


#[component]
pub fn CreateHaiku(mut on_created: impl FnMut(String) + 'static) -> impl IntoView {
    let (haiku_url, set_haiku_url) = signal(Option::<String>::None);
    let (create_loading, set_create_loading) = signal(false);

    let (line_one, set_line_one) = signal(String::new());
    let (line_two, set_line_two) = signal(String::new());
    let (line_three, set_line_three) = signal(String::new());

    Effect::new(move |_| {
        if let Some(url) = haiku_url.get() {
            // use callback to send URL back to the parent component
            on_created(url);
        }
    });   

    view! {
        <div class="max-w-7xl mx-auto flex justify-center items-center h-screen" style="font-family: 'Zen Old Mincho';">
            <div class="relative">
                <div class="absolute -left-20 top-0 text-4xl font-black text-black space-y-1">
                    <p style="writing-mode:vertical-rl" lang="ja">俳句を作ろう</p>
                </div>
                <div>
                    <div class="relative">
                        <input 
                            type="text" 
                            placeholder="line 1" 
                            class="absolute w-full h-24 bg-transparent px-4 text-black placeholder-neutral-500 font-serif z-10 focus:outline-none"
                            on:input:target=move |ev| {
                                set_line_one.set(ev.target().value());
                            }
                        />
                        <svg class="w-full h-24" stroke-width="2" viewBox="0 0 579 106" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <line x1="2.99826" y1="7.5" x2="577.998" y2="5.5" stroke="black"/>
                            <line x1="1.0052" y1="3.50003" x2="578.005" y2="9.50003" stroke="black"/>
                            <line x1="576.002" y1="101.5" x2="1.00166" y2="103.5" stroke="black"/>
                            <line x1="577.995" y1="105.5" x2="0.994841" y2="99.4999" stroke="black"/>
                            <line x1="576.5" y1="3.0049" x2="575.5" y2="105.005" stroke="black"/>
                            <line x1="577.5" y1="105.015" x2="574.5" y2="3.0147" stroke="black"/>
                            <line x1="2.24317" y1="103.024" x2="1.757" y2="0.980923" stroke="black"/>
                        </svg>
                    </div>
                    <div class="relative">
                        <input 
                            type="text" 
                            placeholder="line 2" 
                            class="absolute w-full h-24 bg-transparent px-4 text-black placeholder-neutral-500 font-serif z-10 focus:outline-none"
                            on:input:target=move |ev| {
                                set_line_two.set(ev.target().value());
                            }
                        />
                        <svg class="w-full h-24" stroke-width="2" viewBox="0 0 579 106" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <line x1="2.99826" y1="7.5" x2="577.998" y2="5.5" stroke="black"/>
                            <line x1="1.0052" y1="3.50003" x2="578.005" y2="9.50003" stroke="black"/>
                            <line x1="576.002" y1="101.5" x2="1.00166" y2="103.5" stroke="black"/>
                            <line x1="577.995" y1="105.5" x2="0.994841" y2="99.4999" stroke="black"/>
                            <line x1="576.5" y1="3.0049" x2="575.5" y2="105.005" stroke="black"/>
                            <line x1="577.5" y1="105.015" x2="574.5" y2="3.0147" stroke="black"/>
                            <line x1="2.24317" y1="103.024" x2="1.757" y2="0.980923" stroke="black"/>
                        </svg>
                    </div>
                    <div class="relative">
                        <input 
                            type="text" 
                            placeholder="line 3" 
                            class="absolute w-full h-24 bg-transparent px-4 text-black placeholder-neutral-500 font-serif z-10 focus:outline-none"
                            on:input:target=move |ev| {
                                set_line_three.set(ev.target().value());
                            }
                        />
                        <svg class="w-full h-24" stroke-width="2" viewBox="0 0 579 106" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <line x1="2.99826" y1="7.5" x2="577.998" y2="5.5" stroke="black"/>
                            <line x1="1.0052" y1="3.50003" x2="578.005" y2="9.50003" stroke="black"/>
                            <line x1="576.002" y1="101.5" x2="1.00166" y2="103.5" stroke="black"/>
                            <line x1="577.995" y1="105.5" x2="0.994841" y2="99.4999" stroke="black"/>
                            <line x1="576.5" y1="3.0049" x2="575.5" y2="105.005" stroke="black"/>
                            <line x1="577.5" y1="105.015" x2="574.5" y2="3.0147" stroke="black"/>
                            <line x1="2.24317" y1="103.024" x2="1.757" y2="0.980923" stroke="black"/>
                        </svg>
                    </div>
                    <div class="w-full mt-6">
                        <button 
                            on:click=move |_| {
                                set_create_loading.set(true);
                                let line_one = line_one.get_untracked();
                                let line_two = line_two.get_untracked();
                                let line_three = line_three.get_untracked();
                                
                                spawn_local(async move {
                                    match generate_haiku(line_one, line_two, line_three).await {
                                        Ok(url) => {
                                            // Update the local signal with the URL
                                            set_haiku_url.set(Some(url));
                                            set_create_loading.set(false);
                                        },
                                        Err(e) => {
                                            println!("Failed to generate haiku: {:?}", e);
                                            set_create_loading.set(false);
                                        }
                                    }
                                });
                            }
                            type="button" 
                            class="bg-[url('/assets/img/button_bg.png')] py-3 w-full bg-cover bg-center bg-no-repeat font-bold text-2xl focus:outline-none"
                            class=(["text-neutral-300", "cursor-not-allowed"], move || create_loading.get())
                            class=(["cursor-pointer", "text-white"], move || !create_loading.get())
                            disabled=move || create_loading.get()
                        >
                            Create
                        </button>
                    </div>
                </div>
            </div>
        </div> 
    }
}
