// syntax: f(c) = Y.
// capital letters are only for variables.

pub struct Equation {
    pub lhs: Term,
    pub rhs: Term,
}

pub enum Term {
    Variable(String),

    // constants are just nullary functions.
    Function(String, Vec<Term>),
}
