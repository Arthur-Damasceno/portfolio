use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <div class="about">
            <p class="info">{ "Brazilian 🇧🇷" }</p>
            <p class="info">{ "17 years old 🎂" }</p>
            <p class="info">{ "Studying Mechatronics Engineering at IFCE ⚙️" }</p>
        </div>
    }
}
