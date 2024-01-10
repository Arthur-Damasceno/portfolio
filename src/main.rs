use yew::{prelude::*, Renderer};

#[function_component]
fn App() -> Html {
    let hello_world = "Hello World!";

    html! {
        <div>
            <p>{ hello_world }</p>
        </div>
    }
}

fn main() {
    Renderer::<App>::new().render();
}
