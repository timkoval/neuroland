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
                <div class="flex flex-col justify-start transition-colors duration-200">
                    <div class="flex flex-col md:flex-row justify-start md:space-x-6">
                            <a
                                class="flex flex-row items-center text-gruvbox-fg3 md:hover:text-gruvbox-green active:text-gruvbox-green transition-colors duration-200 mb-6"
                                href="https://neurogarden.timkoval.rs/"
                            >
                                <div class="flex flex-col items-start">
                                    <p class="tracking-wide md:text-2xl text-xl">"neurogarden"</p>
                                    <p class="text-start md:w-full w-48">"garden for thoughts and notes"</p>
                                </div>
                            </a>
                            <a
                                class="flex flex-row items-center text-gruvbox-fg3 md:hover:text-gruvbox-green active:text-gruvbox-green transition-colors duration-200 mb-6"
                                href="https://neurogarden.timkoval.rs/"
                            >
                                <div class="flex flex-col items-start space-y-1">
                                    <p class="tracking-wide md:text-2xl text-xl">"neurogarden"</p>
                                    <p class="text-start md:w-full w-48">"garden for thoughts and notes"</p>
                                </div>
                            </a>
                    </div>
                    <div class="flex flex-col md:flex-row justify-start md:space-x-6">
                            <a
                                class="flex flex-row items-center text-gruvbox-fg3 md:hover:text-gruvbox-green active:text-gruvbox-green transition-colors duration-200 mb-6"
                                href="https://neurogarden.timkoval.rs/"
                            >
                                <div class="flex flex-col items-start">
                                    <p class="tracking-wide md:text-2xl text-xl">"neurogarden"</p>
                                    <p class="text-start md:w-full w-48">"garden for thoughts and notes"</p>
                                </div>
                            </a>
                            <a
                                class="flex flex-row items-center text-gruvbox-fg3 md:hover:text-gruvbox-green active:text-gruvbox-green transition-colors duration-200 mb-6"
                                href="https://neurogarden.timkoval.rs/"
                            >
                                <div class="flex flex-col items-start space-y-1">
                                    <p class="tracking-wide md:text-2xl text-xl">"neurogarden"</p>
                                    <p class="text-start md:w-full w-48">"garden for thoughts and notes"</p>
                                </div>
                            </a>
                    </div>
                </div>
            </div>
        </div>
    }
}
