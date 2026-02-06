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

mod unify;
pub use unify::*;

mod deduce;
pub use deduce::*;

mod rename;
pub use rename::*;

mod pos;
pub use pos::*;

fn main() {
    let state = vec![
        Equation::parse("m(e,X) = X").unwrap(),
        Equation::parse("m(n(X),X) = e").unwrap(),
        Equation::parse("m(m(X,Y),Z) = m(X,m(Y,Z))").unwrap(),
        Equation::parse("a = m(d,e)").unwrap(),
        Equation::parse("b = d").unwrap(),
        // We are looking for 'a = b'.
    ];
    kbc(state);
}
