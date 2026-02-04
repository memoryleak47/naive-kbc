use crate::*;

use std::fmt::Display;

// syntax: f(c) = Y.
// capital letters are only for variables.

#[derive(PartialEq, Eq, Clone)]
pub struct Equation {
    pub lhs: Term,
    pub rhs: Term,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Term {
    Var(Symbol),

    // constants are just nullary functions.
    Fun(Symbol, Box<[Term]>),
}

impl Term {
    pub fn var(x: impl Display) -> Term {
        let x = gsymb_add(x.to_string());
        Term::Var(x)
    }

    pub fn fun(x: impl Display, it: impl IntoIterator<Item=Term>) -> Term {
        let x = gsymb_add(x.to_string());
        let children = it.into_iter().collect();
        Term::Fun(x, children)
    }

    pub fn cst(x: impl Display) -> Term {
        let x = gsymb_add(x.to_string());
        Term::Fun(x, Box::new([]))
    }
}
