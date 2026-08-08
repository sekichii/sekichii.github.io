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
                <p class="info">{ "Systems - Graphics - Low-level - Experiments"}</p>
                <ul>
                    <li class="base">{ "Computer science student with a focus on systems and low-level work" }</li>
                    <li class="base">{ "Passion for graphics, emulators and high-performance computing" }</li>
                    <li class="base">{ "Likes to dive deep into rabbit holes regarding SIMD and Graphics APIs" }</li>
                </ul>
                <p class="base">{ "Demo projects coming soon to this site!" }</p>
            </div>
        </div>
    }
}