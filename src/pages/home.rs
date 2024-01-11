use yew::prelude::*;

use crate::components::{Author, About};

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <Author />
            <About />
        </>
    }
}
