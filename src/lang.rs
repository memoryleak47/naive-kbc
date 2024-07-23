// syntax: f(c) = Y.
// capital letters are only for variables.

#[derive(PartialEq, Eq)]
pub struct Equation {
    pub lhs: Term,
    pub rhs: Term,
}

#[derive(PartialEq, Eq)]
pub enum Term {
    Variable(String),

    // constants are just nullary functions.
    Function(String, Vec<Term>),
}
