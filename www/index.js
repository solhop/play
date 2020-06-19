import * as wasm from "play";

require('./styles.scss');

wasm.greet();

window.solve = function () {
    let input = document.querySelector("#input");
    let output = document.querySelector("#output");
    output.value = wasm.sat_solver(input.value);
}
