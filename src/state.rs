use crate::*;

pub type Equation = (/*lhs*/ Term, /*rhs*/ Term, /*is oriented*/ bool);

pub type State = Vec<Equation>;
