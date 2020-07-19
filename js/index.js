import './styles.scss';

async function main() {
    let wasm = await import("../pkg/index.js").catch(console.error);

    window.solve = function () {
        let input = document.querySelector("#input");
        let output = document.querySelector("#output");
        output.value = wasm.sat_solver(input.value);
    }
}


main();
