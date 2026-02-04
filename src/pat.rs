use crate::*;

pub type Subst = BTreeMap<Symbol, Term>;

// assumption: pat and t have disjoint sets of vars.
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

// assumption: l and r have disjoint sets of vars.
pub fn unify(l: &Term, r: &Term) -> Option<Subst> {
    let mut subst = Default::default();
    unify_impl(l, r, &mut subst)?;
    Some(subst)
}

fn unify_impl(l: &Term, r: &Term, subst: &mut Subst) -> Option<()> {
    // replace defined vars.
    if let Term::Var(lv) = l && let Some(lt) = subst.get(lv) {
        return unify_impl(&lt.clone(), r, subst);
    }
    if let Term::Var(rv) = r && let Some(rt) = subst.get(rv) {
        return unify_impl(l, &rt.clone(), subst);
    }

    // define vars.
    if let Term::Var(lv) = l && subst.get(lv).is_none() {
        subst.insert(*lv, r.clone());
        return unify_impl(l, r, subst);
    }
    if let Term::Var(rv) = r && subst.get(rv).is_none() {
        subst.insert(*rv, l.clone());
        return unify_impl(l, r, subst);
    }

    let Term::Fun(lf, largs) = l else { unreachable!() };
    let Term::Fun(rf, rargs) = r else { unreachable!() };

    if lf != rf { return None }
    if largs.len() != rargs.len() { return None }
    for (ll, rr) in largs.iter().zip(rargs.iter()) {
        unify_impl(ll, rr, subst)?;
    }
    Some(())
}


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
