use std::collections::{BTreeMap, BTreeSet};

mod lang;
pub use lang::*;

mod state;
pub use state::*;

mod order;
pub use order::*;

mod symbol;
pub use symbol::*;

mod parse;
pub use parse::*;

fn main() {
    let fx: Term = Term::parse("f(X)").unwrap();
    let y: Term = Term::parse("Y").unwrap();
    dbg!(fx.partial_cmp(&y));
}
