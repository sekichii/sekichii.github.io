use yew::{component, Html, use_state, html};

use super::icons::*;

#[component]
pub fn AppLegend() -> Html {
    html! {
        <div class="legend">
            <div class="legend-mention">
                <a>{ "Built with " }</a>
                <RustIcon />
            </div>
            <div class="legend-links">
                <a>{"Games"}</a>
                <a>
                    <GithubIcon />
                </a>
                <a>
                    <BlueskyIcon />
                </a>
            </div>
        </div>
    }
}