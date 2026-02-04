use crate::*;

pub type Subst = BTreeMap<Symbol, Term>;

pub fn pat_match(pat: &Term, t: &Term) -> Subst {
    todo!()
}

pub fn unify(pat: &Term, t: &Term) -> Subst { todo!() }
