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
    let state = vec![
        Equation::parse("f(X) = X").unwrap()
    ];
    let state = kbc(state);

    dump_state(&state);
}
