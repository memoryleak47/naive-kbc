use std::collections::{BTreeMap, BTreeSet};

mod lang;
pub use lang::*;

mod state;
pub use state::*;

mod order;
pub use order::*;

mod symbol;
pub use symbol::*;

fn main() {
    // f(X)
    let fx: Term = Term::fun("f", [Term::var("X")]);

    // X
    let x: Term = Term::var("Y");
    dbg!(fx.partial_cmp(&x));
}
