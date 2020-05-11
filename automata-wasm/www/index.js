import { WasmAutomata1D } from "automata-wasm";

// Number of lines drawn per animation frame
const TIMESTEP = 2;
const OUTLINE_COLOR = "#888888";

const canvas = document.getElementById("automata_canvas");
const nb_colors_select = document.getElementById("nbColors")
const rule_nb_input = document.getElementById("ruleNb")
const random_button = document.getElementById("random")
const playpause_button = document.getElementById("play-pause")
const width_input = document.getElementById("width")
const height_input = document.getElementById("height")
const steps_label = document.getElementById("steps")
const ctx = canvas.getContext("2d");

let cur_row = 0;
let width = 1024;
let height = 512;
let nColors = 3;
let ruleNb = BigInt(40327);
let steps = 0;
let need_reset = false;

let automata = null;

const reset_automata = () => {
    automata = WasmAutomata1D.new(nColors, ruleNb, width);
    cur_row = 0;
    steps = 0;
    need_reset = false;
    canvas.height = height + 2;
    canvas.width = width + 2;
    console.log("Width: %d , Height: %d", width, height);
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.beginPath();
    ctx.strokeStyle = OUTLINE_COLOR;
    ctx.moveTo(0, 0); ctx.lineTo(0, height + 2);
    ctx.moveTo(0, 0); ctx.lineTo(width + 2, 0);
    ctx.moveTo(width + 2, height + 2); ctx.lineTo(0, height + 2);
    ctx.moveTo(width + 2, height + 2); ctx.lineTo(width + 2, 0);
    ctx.stroke();

}

const scroll_canvas = () => {
    let image = ctx.getImageData(1, TIMESTEP + 1, width, height - TIMESTEP);
    ctx.putImageData(image, 1, 1);
    cur_row = cur_row - TIMESTEP;
};

const update_steps = () => {
    steps_label.textContent = steps;
}
const draw_steps = () => {
    if (cur_row == height) { scroll_canvas() }
    var vec = automata.step(TIMESTEP);
    var imageData = new ImageData(new Uint8ClampedArray(vec), width, TIMESTEP);
    ctx.putImageData(imageData, 1, cur_row + 1);
    cur_row += TIMESTEP;
    steps += TIMESTEP;
    update_steps();
}
const update_rule_nb = () => {
    var max_nb = WasmAutomata1D.get_max_rule_nb(nColors);
    if (ruleNb > max_nb) {
        ruleNb = max_nb;
    }
    rule_nb_input.value = ruleNb.toString();
    need_reset = true;
}
const update_width = () => {
    width = Number(width_input.value);
    width_input.value = width;
    need_reset = true;
}
const update_height = () => {
    height = Number(height_input.value);
    height_input.value = height;
    need_reset = true;
}
let animation_id = null;
const play = () => {
    if (need_reset) {
        reset_automata();
    }
    playpause_button.textContent = "pause";
    render_loop();
}
const pause = () => {
    cancelAnimationFrame(animation_id);
    playpause_button.textContent = "play";
    animation_id = null;
}
const is_paused = () => {
    return animation_id == null;
}
const render_loop = () => {
    draw_steps();
    animation_id = requestAnimationFrame(render_loop);
}
nb_colors_select.addEventListener("change", event => {
    nColors = nb_colors_select.value;
    update_rule_nb();
})
rule_nb_input.addEventListener("input", event => {
    ruleNb = BigInt(rule_nb_input.value);
    update_rule_nb();
})
width_input.addEventListener("input", event => {
    update_width();
})
height_input.addEventListener("input", event => {
    update_height();
})
playpause_button.addEventListener("click", event => {
    if (is_paused()) {
        play();
    } else {
        pause();
    }
})
random_button.addEventListener("click", event => {
    var max_nb = WasmAutomata1D.get_max_rule_nb(nColors);
    console.assert(max_nb + 1n < BigInt(Number.MAX_SAFE_INTEGER));
    ruleNb = BigInt(Math.floor(Math.random() * Math.floor(Number(max_nb) + 1)));
    update_rule_nb()
})

width_input.value = width;
height_input.value = height;
nb_colors_select.value = nColors;
playpause_button.textContent = "play";
random_button.textContent = "random";

reset_automata();
update_steps();
update_rule_nb();
