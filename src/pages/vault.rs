use leptos::*;

#[component]
pub fn Vault() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            // Greeting
            <div class="flex flex-col md:space-y-8 space-y-6">
                <div class="flex flex-col px-8 text-gruvbox-gray">
                    <p>"Here is a vault of things that I've created."</p>
                    <p>"Click through their titles to get further."</p>
                </div>
            </div>
            // Main Content
            <div
                class="flex flex-grow mb-auto justify-start items-center"
            >
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                            <a
                                class="items-center text-gruvbox-fg3 md:hover:text-gruvbox-yellow2 active:text-gruvbox-yellow2 transition-colors duration-200"
                                href="https://timkoval.rs/"
                            >
                                <div class="flex flex-col items-start">
                                    <p class="tracking-wide md:text-2xl text-xl">"this website"</p>
                                    <p class="text-start md:w-72 w-48">"personal page made with Leptos, Trunk and Tailwind"</p>
                                </div>
                            </a>
                            <a
                                class="items-center text-gruvbox-fg3 md:hover:text-gruvbox-yellow2 active:text-gruvbox-yellow2 transition-colors duration-200"
                                href="https://github.com/timkoval/neuronix"
                            >
                                <div class="flex flex-col items-start space-y-1">
                                    <p class="tracking-wide md:text-2xl text-xl">"neuronix"</p>
                                    <p class="text-start md:w-72 w-48">"Nix(OS|Darwin) config"</p>
                                </div>
                            </a>
                            <a
                                class="items-center text-gruvbox-fg3 md:hover:text-gruvbox-yellow2 active:text-gruvbox-yellow2 transition-colors duration-200"
                                href="https://github.com/timkoval/neuronvim"
                            >
                                <div class="flex flex-col items-start space-y-1">
                                    <p class="tracking-wide md:text-2xl text-xl">"neuronvim"</p>
                                    <p class="text-start md:w-72 w-48">"Neovim lightweight config"</p>
                                </div>
                            </a>
                            <a
                                class="items-center text-gruvbox-fg3 md:hover:text-gruvbox-yellow2 active:text-gruvbox-yellow2 transition-colors duration-200"
                                href="https://github.com/timkoval/aoc-2023"
                            >
                                <div class="flex flex-col items-start space-y-1">
                                    <p class="tracking-wide md:text-2xl text-xl">"aoc-2023"</p>
                                    <p class="text-start md:w-72 w-48">"Advent of Code 2023 in Rust"</p>
                                </div>
                            </a>
                </div>
            </div>
        </div>
    }
}
