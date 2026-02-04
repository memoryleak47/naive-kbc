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
    // let t = |x| Term::parse(x).unwrap();
    // dbg!(t("f(X)").partial_cmp(&t("Y")));
    // dbg!(t("f(X)").partial_cmp(&t("X")));
    // dbg!(t("f(a, X)").partial_cmp(&t("X")));
}
