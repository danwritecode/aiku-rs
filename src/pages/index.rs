use leptos::prelude::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="bg-[url('/assets/img/landing_background.png')] bg-cover bg-center bg-no-repeat h-screen">
            <div class="max-w-7xl mx-auto grid grid-cols-1 lg:grid-cols-3 gap-4 h-full" style="font-family: 'Zen Old Mincho';">
                <div class="col-span-1 flex flex-col items-center pt-56 text-7xl font-black text-red-900 space-y-1">
                    <p>俳</p>
                    <p>句</p>
                </div>
                <div class="col-span-1 flex flex-col items-center justify-center">
                    <div class="relative">
                        <input 
                            type="text" 
                            placeholder="line 1" 
                            class="absolute w-full h-24 bg-transparent px-4 text-black placeholder-neutral-500 font-serif z-10 focus:outline-none"
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
                            type="button" 
                            class="bg-[url('/assets/img/button_bg.png')] cursor-pointer py-3 w-full bg-cover bg-center bg-no-repeat text-white font-bold text-2xl focus:outline-none"
                        >
                            Create
                        </button>
                    </div>
                </div>
            </div> 
        </div>
    }
}
