use crate::*;

pub type Subst = BTreeMap<Symbol, Term>;

pub fn apply_subst(t: &Term, subst: &Subst) -> Term {
    match t {
        Term::Var(v) => {
            if let Some(t2) = subst.get(v) {
                t2.clone()
            } else {
                Term::Var(*v)
            }
        },
        Term::Fun(f, args) => {
            let args = args.iter().map(|x| apply_subst(x, subst)).collect();
            Term::Fun(*f, args)
        },
    }
}

pub fn v_disjoint(l1: &BTreeMap<Symbol, usize>, l2: &BTreeMap<Symbol, usize>) -> bool {
    l1.keys().all(|x| !l2.contains_key(x))
}

// pat and t are not allowed to share variables.
// (otherwise the 'subst' can create cyclic simplifications)
pub fn pat_match(pat: &Term, t: &Term) -> Option<Subst> {
    let pat_vars = get_vars(pat);
    let t_vars = get_vars(t);
    assert!(v_disjoint(&pat_vars, &t_vars));

    let mut subst = Default::default();
    pat_match_impl(pat, t, &mut subst, &pat_vars)?;
    Some(subst)
}

// subst :: vars(pat) -> Term[vars(t)]
fn pat_match_impl(pat: &Term, t: &Term, subst: &mut Subst, pat_vars: &BTreeMap<Symbol, usize>) -> Option<()> {
    match pat {
        Term::Var(v) => {
            if let Some(tv) = subst.get(v) {
                let tv = tv.clone();
                return pat_match_impl(&tv, t, subst, pat_vars);
            }

            // we only insert stuff into pat-vars, not the non-pat vars that we from a prior subst.
            if pat_vars.contains_key(&v) {
                subst.insert(*v, t.clone());
            } else {
                return match t {
                    Term::Var(vv) if v == vv => Some(()),
                    _ => None,
                };
            }
        },
        Term::Fun(f, args) => {
            let Term::Fun(f2, args2) = t else { return None };
            if f != f2 { return None }
            if args.len() != args2.len() { return None }
            for (x, y) in args.iter().zip(args2.iter()) {
                pat_match_impl(x, y, subst, pat_vars)?;
            }
        },
    }
    Some(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_match1() {
        let pat = Term::parse("f(X)").unwrap();
        let t = Term::parse("f(a)").unwrap();
        let subst = pat_match(&pat, &t).unwrap();
        let a = Term::parse("a").unwrap();

        let mut correct_subst = Subst::default();
        correct_subst.insert(gsymb_add(format!("X")), a);
        assert!(subst == correct_subst);
    }

    #[test]
    fn test_match2() {
        let pat = Term::parse("A").unwrap();
        let t = Term::parse("f(a, f(X, z))").unwrap();
        let subst = pat_match(&pat, &t).unwrap();
        let a = Term::parse("a").unwrap();

        let mut correct_subst = Subst::default();
        correct_subst.insert(gsymb_add(format!("A")), t.clone());
        assert!(subst == correct_subst);
    }
}

// This check could potentially be deprecated by a consistent variable naming.
pub fn literally_similar(l: &Term, r: &Term) -> bool {
    let lvars = get_vars(l);
    let rvars = get_vars(r);
    assert!(v_disjoint(&lvars, &rvars));
    if lvars.len() != rvars.len() { return false }

    let Some(sig) = pat_match(l, r) else { return false };
    let mut v = Vec::new();
    for x in lvars.keys() {
        let Some(Term::Var(vv)) = sig.get(x) else { return false };
        v.push(vv);
    }
    v.sort();
    v.dedup();

    v.len() == lvars.len()
}

#[test]
fn test_pat_match() {
    let p = Term::parse("m(n(A), A)").unwrap();
    let t = Term::parse("m(n(X), m(X, Y))").unwrap();
    assert_eq!(None, pat_match(&p, &t));
}
