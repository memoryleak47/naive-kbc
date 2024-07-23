use crate::*;

pub struct State {
    // from lhs to rhs.
    oriented: Vec<Equation>,

    unoriented: Vec<Equation>,
}

impl State {
    pub fn new(eqs: Vec<Equation>) -> Self {
        State {
            unoriented: eqs,
            oriented: Vec::new(),
        }
    }


    // apply all rules, as long as possible.
    fn tick(&mut self) {
        todo!()
    }

    fn canonicalize(&self, t: Term) -> Term {
        todo!()
    }
}
