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

fn main() {
    let mut state = vec![
        Equation::parse("f(X) = f(f(X))").unwrap()
    ];
    dump_state(&state);

    for _ in 0..10 {
        println!("-------------------------");
        state = kbc_step(state);
        dump_state(&state);
    }
}
