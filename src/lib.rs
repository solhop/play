mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_str(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    // Called to log the panic error messages to console.err
    utils::set_panic_hook();
    log_str("Hello, play!");
}

#[wasm_bindgen]
pub fn sat_solver(input: String) -> String {
    // Called to log the panic error messages to console.err
    utils::set_panic_hook();

    let mut reader = std::io::BufReader::new(input.as_bytes());
    let parsed = rsat::parser::parse_dimacs_from_buf_reader(&mut reader);
    let (n_vars, clauses) = if let rsat::parser::Dimacs::Cnf { n_vars, clauses } = parsed {
        (n_vars, clauses)
    } else {
        panic!("Incorrect input format");
    };

    use rsat::cdcl::*;
    use rsat::Var;

    let options = SolverOptions::default();
    let mut solver = Solver::new(options);

    let vars: Vec<Var> = (0..n_vars).map(|_| solver.new_var()).collect();

    for clause in clauses {
        let lits = clause
            .into_iter()
            .map(|l| {
                let var = vars[(l.abs() - 1) as usize];
                if l < 0 {
                    var.neg_lit()
                } else {
                    var.pos_lit()
                }
            })
            .collect();
        solver.add_clause(lits);
    }

    let solution = solver.solve(vec![]);

    use rsat::Solution::*;
    match solution {
        Unsat => "s UNSATISFIABLE\n".to_owned(),
        Unknown => "s UNKNOWN\n".to_owned(),
        Best(solution) => {
            let mut output = String::new();
            output += "s UNKNOWN\n";
            let solution = solution.iter().map(|&x| if x { 1 } else { -1 });
            output += "v ";
            for (i, v) in solution.enumerate() {
                output += &format!("{} ", v * ((i + 1) as i32));
            }
            output += "0\n";
            output
        }
        Sat(solution) => {
            let mut output = String::new();
            output += "s SATISFIABLE\n";
            output += "v ";
            let solution = solution.iter().map(|&x| if x { 1 } else { -1 });
            for (i, v) in solution.enumerate() {
                output += &format!("{} ", v * ((i + 1) as i32));
            }
            output += "0\n";
            output
        }
    }
}
