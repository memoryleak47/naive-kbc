mod lang;
pub use lang::*;

mod state;
pub use state::*;

mod order;
pub use order::*;

fn main() {
    let mk_var = |x: &str| Term::Variable(x.to_string());
    let mk_fn = |x: &str, args: Vec<Term>| Term::Function(x.to_string(), args);

    // f(X)
    let fx: Term = mk_fn("f", vec![mk_var("X")]);

    // X
    let x: Term = mk_var("Y");
    dbg!(fx.partial_cmp(&x));
}
