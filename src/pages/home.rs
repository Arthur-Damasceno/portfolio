use yew::prelude::*;

use crate::components::{Author, About, GithubRepository};

#[function_component]
pub fn Home() -> Html {
    html! {
        <>
            <GithubRepository />
            <Author />
            <About />
        </>
    }
}
