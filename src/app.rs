use yew::prelude::*;
use crate::pages;
use crate::atom;

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
            <div class="atom-feed">
                { for atom::get_atoms().iter().map(|atom| html! {
                    <div class="atom">
                        <h2 class="atom-title">{ atom.title }</h2>
                        { atom.description.clone() }
                        <span class="atom-date">{ atom.date }</span>
                        <hr />
                    </div>
                }) }
            </div>
        </main>
    }
}
