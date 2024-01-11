use yew::prelude::*;

const GITHUB_REPOSITORY_URL: &str = "https://github.com/Arthur-Damasceno/portfolio";

#[function_component]
pub fn GithubRepository() -> Html {
    html! {
        <div class="container">
            <a href={GITHUB_REPOSITORY_URL} target="blank" class="github-link">
                { "View in Github" }
                <img style="height:7vh" src="assets/github-mark.svg" alt="Github Logo" />
            </a>
        </div>
    }
}
