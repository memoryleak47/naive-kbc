use std::collections::BTreeMap;

mod lang;
pub use lang::*;

mod state;
pub use state::*;

mod simplify;
pub use simplify::*;

mod order;
pub use order::*;

mod symbol;
pub use symbol::*;

mod parse;
pub use parse::*;

mod pat;
pub use pat::*;

mod deduce;
pub use deduce::*;

mod rename;
pub use rename::*;

mod pos;
pub use pos::*;

fn main() {
    let mut state = vec![
        Equation::parse("f(g(X)) = g(f(X))").unwrap(),
        Equation::parse("f(f(Y)) = Y").unwrap(),
        Equation::parse("g(g(Y)) = Y").unwrap(),
        Equation::parse("g(g(A)) = A").unwrap(),
        Equation::parse("Z = f(g(f(g(Z))))").unwrap(),
    ];
    dump_state(&state);

    for _ in 0..10 {
        println!("-------------------------");
        state = kbc_step(state);
        dump_state(&state);
    }
}
