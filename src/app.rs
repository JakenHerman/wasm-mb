use yew::prelude::*;
use crate::pages;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <nav>
                <ul class="navigation">
                    { for pages::PAGES.iter().map(|page| html! {
                        <li><a href={ page.link }>{ page.name }</a></li>
                    }) }
                </ul>
            </nav>
            <h1>{ "Jaken Herman" }</h1>
            <span class="subtitle">{ "Welcome to my page." }</span>
        </main>
    }
}
