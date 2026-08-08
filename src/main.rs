mod components;
use components::{
    legend::AppLegend
};

use yew::{component, Html, use_state, html};

#[component]
pub fn App() -> Html {
    /*let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };*/

    html! {
        <div>
            <AppLegend />
            /*<button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>*/
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}