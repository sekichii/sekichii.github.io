use yew::prelude::*;
use yew::Properties;

#[derive(Properties, PartialEq)]
pub struct ProjectProps {
    #[prop_or(AttrValue::Static("Default"))]
    pub name: AttrValue,
    #[prop_or(AttrValue::Static("Default"))]
    pub description: AttrValue,
    #[prop_or(AttrValue::Static("Default"))]
    pub image_url: AttrValue,
    #[prop_or(AttrValue::Static("Default"))]
    pub url: AttrValue,
}

#[component]
pub fn Project(&ProjectProps { ref name, ref description, ref image_url, ref url }: &ProjectProps) -> Html {
    html! {
        <div class="project">
            <img class="project-image" src={ image_url } />
            <div class="project-info">
                <p class="project-name">{ name }</p>
                <p class="project-description">{ description }</p>
            </div>
            <a class="project-link" href={ url }>{ "Check Out" }</a>
        </div>
    }
}