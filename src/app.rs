use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="font-fira-mono flex flex-col bg-gruvbox-bg min-h-screen text-base text-center text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <Header/>
                <div class="flex flex-1 md:pt-8 pt-6 md-8 md:pb-8 pb-12 md:w-full md:max-w-4xl justify-center">
                    <Home/>
                </div>
                <Footer/>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center justify-center">
            // Greeting
            <div class="flex flex-col md:space-y-8 space-y-6">
                <div class="flex flex-col px-8 text-gruvbox-gray">
                    <p>"It looks like you've stumbled upon my personal website."</p>
                    <p>"Below are some parts of my life."</p>
                </div>
            </div>
            // Main Content
            <div
                class="flex flex-grow mb-auto justify-start items-center"
            >
                <div class="flex flex-col justify-start md:space-y-12 space-y-8 transition-colors duration-200">
                    <a
                        class="flex flex-row items-center md:space-x-8 space-x-6 text-gruvbox-fg3 md:hover:text-gruvbox-green active:text-gruvbox-green transition-colors duration-200"
                        href="https://neurogarden.timkoval.rs/"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36" width="48" height="48"><path fill="#5C913B" d="M36 33.5c0 .828-.672 1.5-1.5 1.5h-33C.672 35 0 34.328 0 33.5S.672 32 1.5 32h33c.828 0 1.5.672 1.5 1.5z"/><path fill="#A0041E" d="M12.344 14.702h-2c-.276 0-.5-.224-.5-.5v-7c0-.276.224-.5.5-.5h2c.276 0 .5.224.5.5v7c0 .276-.224.5-.5.5z"/><path fill="#FFCC4D" d="M5.942 32c-.137-4.657-.506-8-.942-8-.435 0-.804 3.343-.941 8h1.883z"/><path fill="#77B255" d="M10 18.731C10 24.306 7.762 26 5 26c-2.761 0-5-1.694-5-7.269C0 13.154 4 5 5 5s5 8.154 5 13.731z"/><path fill="#FFE8B6" d="M8 16L21 3l13 13v16H8z"/><path fill="#FFCC4D" d="M21 16h1v16h-1z"/><path fill="#66757F" d="M34 17c-.256 0-.512-.098-.707-.293L21 4.414 8.707 16.707c-.391.391-1.023.391-1.414 0s-.391-1.023 0-1.414l13-13c.391-.391 1.023-.391 1.414 0l13 13c.391.391.391 1.023 0 1.414-.195.195-.451.293-.707.293z"/><path fill="#66757F" d="M21 17c-.256 0-.512-.098-.707-.293-.391-.391-.391-1.023 0-1.414l6.5-6.5c.391-.391 1.023-.391 1.414 0s.391 1.023 0 1.414l-6.5 6.5c-.195.195-.451.293-.707.293z"/><path fill="#C1694F" d="M13 26h4v6h-4z"/><path fill="#55ACEE" d="M13 17h4v4h-4zm12.5 0h4v4h-4zm0 9h4v4h-4z"/><path fill="#77B255" d="M10.625 29.991c0 1.613-.858 2.103-1.917 2.103-1.058 0-1.917-.49-1.917-2.103 0-1.613 1.533-3.973 1.917-3.973s1.917 2.359 1.917 3.973zm25.25 0c0 1.613-.858 2.103-1.917 2.103-1.058 0-1.917-.49-1.917-2.103 0-1.613 1.533-3.973 1.917-3.973.384 0 1.917 2.359 1.917 3.973z"/></svg>
                        <div class="flex flex-col items-start space-y-1">
                            <p class="tracking-wide md:text-2xl text-xl">"neurogarden"</p>
                            <p class="text-start md:w-full w-48">"garden for thoughts and notes"</p>
                        </div>
                    </a>
                    <a
                        class="flex flex-row items-center md:space-x-8 space-x-6 text-gruvbox-fg3 md:hover:text-gruvbox-yellow2 active:text-gruvbox-yellow2 transition-colors duration-200"
                        href="https://github.com/timkoval"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36" width="48" height="48"><path fill="#292F33" d="M30 34c0 1.104-.896 2-2 2H8c-1.104 0-2-.896-2-2V2c0-1.104.896-2 2-2h20c1.104 0 2 .896 2 2v32z"/><path fill="#66757F" d="M28 16c0 .552-.447 1-1 1H9c-.552 0-1-.448-1-1V3c0-.552.448-1 1-1h18c.553 0 1 .448 1 1v13zm0 17c0 .553-.447 1-1 1H9c-.552 0-1-.447-1-1V20c0-.553.448-1 1-1h18c.553 0 1 .447 1 1v13z"/><path fill="#292F33" d="M22 8c0 .552-.447 1-1 1h-6c-.552 0-1-.448-1-1V5c0-.552.448-1 1-1h6c.553 0 1 .448 1 1v3zm0 17c0 .553-.447 1-1 1h-6c-.552 0-1-.447-1-1v-3c0-.553.448-1 1-1h6c.553 0 1 .447 1 1v3z"/><path fill="#E1E8ED" d="M15 5h6v3h-6zm0 17h6v3h-6zm9-8.97c0 .536-.435.97-.97.97H12.97c-.536 0-.97-.435-.97-.97v-.06c0-.536.434-.97.97-.97h10.06c.535 0 .97.435.97.97v.06zm0 16.999c0 .536-.435.971-.97.971H12.97c-.536 0-.97-.435-.97-.971v-.059c0-.535.434-.97.97-.97h10.06c.535 0 .97.435.97.971v.058z"/></svg>
                        <div class="flex flex-col items-start space-y-1">
                            <p class="tracking-wide md:text-2xl text-xl">"neurovault"</p>
                            <p class="text-start md:w-full w-48">"vault for projects and ideas"</p>
                        </div>
                    </a>
                    <a
                        class="flex flex-row items-center md:space-x-8 space-x-6 text-gruvbox-fg3 md:hover:text-gruvbox-blue active:text-gruvbox-blue transition-colors duration-200"
                        href="/Tim_Koval_CV.pdf"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 36 36" width="48" height="48"><path fill="#C1694F" d="M32 34c0 1.104-.896 2-2 2H6c-1.104 0-2-.896-2-2V7c0-1.104.896-2 2-2h24c1.104 0 2 .896 2 2v27z"/><path fill="#FFF" d="M29 32c0 .553-.447 1-1 1H8c-.552 0-1-.447-1-1V9c0-.552.448-1 1-1h20c.553 0 1 .448 1 1v23z"/><path fill="#CCD6DD" d="M25 3h-4c0-1.657-1.343-3-3-3s-3 1.343-3 3h-4c-1.104 0-2 .896-2 2v5h18V5c0-1.104-.896-2-2-2z"/><circle fill="#292F33" cx="18" cy="3" r="2"/><path fill="#99AAB5" d="M20 14c0 .552-.447 1-1 1h-9c-.552 0-1-.448-1-1s.448-1 1-1h9c.553 0 1 .448 1 1zm7 4c0 .552-.447 1-1 1H10c-.552 0-1-.448-1-1s.448-1 1-1h16c.553 0 1 .448 1 1zm0 4c0 .553-.447 1-1 1H10c-.552 0-1-.447-1-1 0-.553.448-1 1-1h16c.553 0 1 .447 1 1zm0 4c0 .553-.447 1-1 1H10c-.552 0-1-.447-1-1 0-.553.448-1 1-1h16c.553 0 1 .447 1 1zm0 4c0 .553-.447 1-1 1h-9c-.552 0-1-.447-1-1 0-.553.448-1 1-1h9c.553 0 1 .447 1 1z"/></svg>
                        <div class="flex flex-col items-start space-y-1">
                            <p class="tracking-wide md:text-2xl text-xl">"neurocv"</p>
                            <p class="text-start md:w-full w-48">"personal CV (more like a resume)"</p>
                        </div>
                    </a>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <div class="tracking-widest mt-4 flex sm:text-lg text-gruvbox-green justify-center items-center inset-x-0 top-0 bottom-auto">"Hi! I'm Tim Koval."</div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="pb-0 m-auto left-0 right-0">
            <a href="https://github.com/timkoval" target="_blank" class="text-xs">
                "2024 timkoval"
            </a>
        </footer>
    }
}
