mod components;
mod views;

use views::{
    about::AboutMe,
    legend::Legend,
    project::{ProjectProps, Project}
};

use yew::{component, Html, use_state, html};

#[component(Main)]
pub fn app() -> Html {
    html! {
        <div>
            <Legend />
            <AboutMe />
            //<Project name="hello" description="hello" url="http://hi.com" image_url="assets/profile.jpg" />
        </div>
    }
}

fn main() {
    yew::Renderer::<Main>::new().render();
}