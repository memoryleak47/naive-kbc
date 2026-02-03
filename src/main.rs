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
    // let t = |x| Term::parse(x).unwrap();
    // dbg!(t("f(X)").partial_cmp(&t("Y")));
    // dbg!(t("f(X)").partial_cmp(&t("X")));
    // dbg!(t("f(a, X)").partial_cmp(&t("X")));
}
