use crate::*;

pub fn simplify_single(mut term: Term, eq: &Equation) -> Term {
    // root level application
    if let Some(subst) = pat_match(&eq.0, &term) {
        term = apply_subst(&eq.1, &subst);
    }
    match term {
        Term::Fun(f, args) => {
            let args = args.into_iter().map(|x| simplify_single(x, eq)).collect();
            Term::Fun(f, args)
        }
        term => term,
    }
}
