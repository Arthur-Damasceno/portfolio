mod components;

use yew::{prelude::*, Renderer};
use components::*;

#[function_component]
fn App() -> Html {
    html! {
        <Author />
    }
}

fn main() {
    Renderer::<App>::new().render();
}
