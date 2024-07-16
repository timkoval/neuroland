use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::home::*;
use crate::pages::vault::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    let title_formatter = |text| format!("{text} - timkoval.rs");

    view! {
        <Title formatter=title_formatter/>

        <div class="font-fira-mono flex flex-col bg-gruvbox-bg min-h-screen text-base text-center text-gray-300">
            <div class="flex flex-1 flex-col items-center">
                <Header/>
                <div class="flex flex-1 md:pt-8 pt-6 md-8 md:pb-8 pb-12 md:w-full md:max-w-4xl justify-center">
                    <Router>
                        <Routes>
                            <Route path="" view=  move || view! { <Home /> }/>
                            <Route path="/neurovault" view=  move || view! { <Vault /> }/>
                        </Routes>
                    </Router>
                </div>
                <Footer/>
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
        <footer class="pb-0 m-auto left-0 right-0 text-gruvbox-gray">
            <a href="https://github.com/timkoval" target="_blank" rel="noopener noreferrer" class="text-xs">
                "2024 timkoval "
            </a>
            <a href="mailto: timkoval00@gmail.com" class="text-xs text-blue-600 hover:text-blue-800">
                " (click here to contact me by email)"
            </a>
        </footer>
    }
}
