use crate::*;

use std::fmt::Display;

// syntax: f(c) = Y.
// capital letters are only for variables.

#[derive(PartialEq, Eq)]
pub struct Equation {
    pub lhs: Term,
    pub rhs: Term,
}

#[derive(PartialEq, Eq)]
pub enum Term {
    Variable(Id),

    // constants are just nullary functions.
    Function(Id, Box<[Term]>),
}

impl Term {
    pub fn var(x: impl Display) -> Term {
        let x = gsymb_add(x.to_string());
        Term::Variable(x)
    }

    pub fn fun(x: impl Display, it: impl IntoIterator<Item=Term>) -> Term {
        let x = gsymb_add(x.to_string());
        let children = it.into_iter().collect();
        Term::Function(x, children)
    }
}
