use crate::*;

use std::cmp::*;

const VAR_WEIGHT: usize = 2;
const SYM_WEIGHT: usize = 1;

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        if self == other { return Some(Ordering::Equal); }

        // TODO incorrect. consider f(X) vs Y.
        let out = term_weight(self).cmp(&term_weight(other));
        if out != Ordering::Equal { return Some(out); }

        match (self, other) {
            // f(l1, l2, l3) < g(r1, r2, r3), iff
            // f < g, or f=g and ...
            // l1 < r1, or l1=r1 and ...
            (Term::Function(f1, args1), Term::Function(f2, args2)) => {
                let o = f1.cmp(f2);
                if o != Ordering::Equal { return Some(o); }
                assert_eq!(args1.len(), args2.len());
                for (l, r) in args1.iter().zip(args2.iter()) {
                    // if the next relevant subterms are unrelated, so are the superterms.
                    let o = l.partial_cmp(r)?;
                    if o != Ordering::Equal {
                        return Some(o);
                    }
                }
                Some(Ordering::Equal)
            },

            // comparing anything with at least one variable cannot be answered, as you might insert arbitrarily big / small things into that variable.
            _ => return None,
        }
    }
}

fn term_weight(t: &Term) -> usize {
    match t {
        Term::Variable(_) => VAR_WEIGHT,
        Term::Function(_, args) => args.iter().map(term_weight).sum::<usize>() + SYM_WEIGHT,
    }
}
