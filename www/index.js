import * as wasm from "play";

wasm.greet();

let input = "p cnf 2 3\n1 0\n-1 2 0\n2 0\n";
console.log(wasm.sat_solver(input));
