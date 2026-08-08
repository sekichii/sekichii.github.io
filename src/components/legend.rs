use yew::{component, Html, use_state, html};

use super::icons::*;

#[component]
pub fn AppLegend() -> Html {
    html! {
        <div class="legend">
            <a>{"Games"}</a>
            <a>
                <GithubIcon />
            </a>
            <a>
                <BlueskyIcon />
            </a>
        </div>
    }
}