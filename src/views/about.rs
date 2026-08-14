use yew::prelude::*;

#[component]
pub fn AboutMe() -> Html {
    html! {
        <div class="about-me">
            <div class="wrapper-left">
                <img src="/assets/profile.jpg" />
            </div>
            <div class="wrapper-right">
                <h1>{ "Hi, I am " }<span class="name">{ "Shina" }</span></h1>
                <p class="info">{ "Systems - Graphics - Low-level - SIMD" }</p>
                <p class="base">
                    { "I am super interested in systems programming and low-level systems.
                        I like to understand things from the ground up, because it gives me
                        a deeper understanding and control over the whole system.
                        I also really enjoy optimizing hot paths, and working in projects
                        with immediate, tangible feedback." }
                    /*{ "I am super interested in systems programming and low-level systems." } <br />
                    { "I like to understand things from the ground up, because it gives me" } <br />
                    { "a deeper understanding and control over the whole system." } <br />
                    { "I also really enjoy optimizing hot paths, and working in projects" } <br />
                    { "with immediate, tangible feedback." }*/
                </p>
            </div>
        </div>
    }
}