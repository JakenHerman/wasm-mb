use yew::prelude::*;

// create a struct to hold micro-blog information
#[derive(Clone)]
pub struct Atom {
    pub title: &'static str,
    pub description: Html,
    pub date: &'static str,
}

// create a function to return a vector of Atoms
pub fn get_atoms() -> Vec<Atom> {
    let mut atom = Vec::new();
    atom.push(Atom {
        title: "📢 Good enough for now",
        description: html! {
            <div class="atom-desc">{
                "I think eventually I'll make a data store for this and populate it differently,
                maybe even with some sort of hook to tweet my micro-blog posts, but for now - this
                is pretty neat. I'm really just playing around with Yew + Rust + WASM + Trunk."
            }</div>
        },
        date: "2022-09-16",
    });
    atom.push(Atom {
        title: "📢 Testing Atom Feed",
        description: html! {
            <div class="atom-desc">{ "This is a test of the atom feed." }</div>
        },
        date: "2022-09-16",
    });
    atom
}