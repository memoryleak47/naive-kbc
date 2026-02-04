use crate::*;

pub fn simplify_single(term: Term, eq: &Equation) -> Term {
    // TODO iterate through subpositions
    if let Some(subst) = pat_match(&eq.0, &term) {
        return apply_subst(&eq.1, &subst);
    }
    term
}

pub fn simplify(mut term: Term, state: &State) -> Term {
    for rw@(l, r, ori) in state {
        if !ori { continue }
        term = simplify_single(term, rw);
    }
    term
}
