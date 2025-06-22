var isDark = true;
var isHidden = true;
var about_me;
var color_button;
var about_me_div;
var color_bg;

const light_color = "#dfdeda";
const dark_color = "#000b14"
const dark_slash = "#041b38";
const light_slash = "#ff5733";

document.addEventListener("DOMContentLoaded", function() {
    color_button = document.getElementById("color_button");
    color_bg = document.getElementById("color_bg");
});

/*function about_me_page(event) {
    if (isHidden) {
        $("#about_me_div").css("visibility", "visible");
        $("#about_me_div").css("opacity", 1);
        isHidden = false;
    } else {
        $("#about_me_div").css("visibility", "hidden");
        $("#about_me_div").css("opacity", 0);
        isHidden = true;
    }
}*/


function color_change(event) {
    if (isDark) {
        color_button.innerHTML = "<span id='color_bg'>LGT</span>DRK";
        $("#color_bg").css("background-color", light_slash);
        $("#color_bg").css("color", light_color);
        $("#falling_blocks").css("background-color", light_color);
        $(".block").css("background-color", light_slash);
        $("body").css("background-color", light_color);
        $("body").css("color", dark_color);
        $("a").css("color", dark_color);
        $(".slidey").css("background-color", "rgba(255, 255, 255, 0.05)");
        $(".slidey .slidey-line").css("color", light_slash);

        isDark = false;
    } else {
        color_button.innerHTML = "LGT<span id='color_bg'>DRK</span>";
        $("#color_bg").css("background-color", light_color);
        $("#color_bg").css("color", dark_color);
        $("#falling_blocks").css("background-color", dark_color);
        $(".block").css("background-color", dark_slash);
        $("body").css("background-color", dark_color);
        $("body").css("color", light_color);
        $("a").css("color", light_color);
        $(".slidey").css("background-color", "rgba(0, 0, 0, 0.05)");
        $(".slidey .slidey-line").css("color", dark_slash);
        $(".highlight").css("background-color", light_slash);
        $(".highlight").css("color", dark_color);

        isDark = true;
    }
}

