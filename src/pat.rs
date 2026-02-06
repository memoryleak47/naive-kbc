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

// pat and t might have shared variables among them.
// TODO: having shared vars between 'pat' and 't' causes infinite loops, as applying the subst can loop.
pub fn pat_match(pat: &Term, t: &Term) -> Option<Subst> {
    let mut subst = Default::default();
    pat_match_impl(pat, t, &mut subst)?;
    Some(subst)
}

// subst :: vars(pat) -> Term[vars(t)]
fn pat_match_impl(pat: &Term, t: &Term, subst: &mut Subst) -> Option<()> {
    match pat {
        Term::Var(v) => {
            if let Some(tv) = subst.get(v) {
                let tv = tv.clone();
                return pat_match_impl(&tv, t, subst);
            }

            subst.insert(*v, t.clone());
        },
        Term::Fun(f, args) => {
            let Term::Fun(f2, args2) = t else { return None };
            if f != f2 { return None }
            if args.len() != args2.len() { return None }
            for (x, y) in args.iter().zip(args2.iter()) {
                pat_match_impl(x, y, subst)?;
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
