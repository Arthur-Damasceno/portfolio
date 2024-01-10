use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    html! {
        <p class="text">{ "Hi 👋! I'm Arthur Damasceno." }</p>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
