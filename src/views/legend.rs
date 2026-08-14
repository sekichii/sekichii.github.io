use yew::{component, Html, use_state, html};

use crate::components::{
    icons::{RustIcon, GithubIcon, BlueskyIcon}
};

#[component]
pub fn Legend() -> Html {
    html! {
        <div class="legend">
            <div class="legend-mention">
                <a>{ "Built with " }</a>
                <RustIcon />
            </div>
            <div class="legend-links">
                <a href="https://github.com/sekichii">
                    <GithubIcon />
                </a>
                <a href="https://shina010.bsky.social">
                    <BlueskyIcon />
                </a>
            </div>
        </div>
    }
}