var isEnglish = true;
var isHidden = true;
var about_me;
var language_button;
var about_me_div;

document.addEventListener("DOMContentLoaded", function() {
    about_me = document.getElementById("about_me");
    language_button = document.getElementById("language_button");

    about_me_div = document.getElementById("about_me_div");
    about_me_div.style.visibility = "hidden";
    about_me_div.style.opacity = "0";

    const title = document.getElementById("title");
    const titleText = "Benih.";

    let i = 0;
    const typingSpeed = 50; 

    function typeText() {
        if (i < titleText.length) {
            title.innerHTML = "<h1>B" + titleText.substring(1, i+1) + "</h1>";
            i++;
            setTimeout(typeText, typingSpeed);
        }
    }

    typeText();
});

function about_me_page(event) {
    if (isHidden) {
        about_me_div.style.visibility = "visible";
        about_me_div.style.opacity = "1";
        isHidden = false;
    } else {
        about_me_div.style.visibility = "hidden";
        about_me_div.style.opacity = "0";
        isHidden = true;
    }
}

function language_change(event) {
    if (isEnglish) {
        language_button.innerHTML = "<span id='lang'>HU</span>EN";

        past_projects.innerHTML = "Előző projektek";
        about_me.innerHTML = "Én rólam";
        isEnglish = false;
    } else {
        language_button.innerHTML = "HU<span id='lang'>EN</span>";

        past_projects.innerHTML = "Past projects";
        about_me.innerHTML = "About me";
        isEnglish = true;
    }
}
