mod components;
mod pages;

use yew::{prelude::*, Renderer};

use pages::*;

#[function_component]
fn App() -> Html {
    html! {
        <Home />
    }
}

fn main() {
    Renderer::<App>::new().render();
}
