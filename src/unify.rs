use crate::*;

// assumption: l and r have disjoint sets of vars.
pub fn unify(l: &Term, r: &Term) -> Option<Subst> {
    let mut subst = Default::default();
    unify_impl(l, r, &mut subst)?;
    Some(subst)
}

// invariants: subst is always fully simplified w.r.t. itself.
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
        subst_add(*lv, r.clone(), subst)?;
        return unify_impl(l, r, subst);
    }
    if let Term::Var(rv) = r && subst.get(rv).is_none() {
        subst_add(*rv, l.clone(), subst)?;
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

fn subst_add(v: Symbol, t: Term, subst: &mut Subst) -> Option<()> {
    let t = fix_apply_subst(t, subst);

    // nothing to be added.
    if t == Term::Var(v) { return Some(()) }

    // cyclic definition, forbidden!
    if contains_var(&t, v) { return None }

    subst.insert(v, t);

    let old_subst = subst.clone();

    for (vv, tt) in subst.iter_mut() {
        *tt = fix_apply_subst(tt.clone(), &old_subst);
    }

    Some(())
}

fn fix_apply_subst(mut t: Term, subst: &Subst) -> Term {
    loop {
        let t2 = apply_subst(&t, subst);
        if t == t2 { return t }
        t = t2;
    }
}

fn contains_var(t: &Term, v: Symbol) -> bool {
    match t {
        Term::Var(v2) => v == *v2,
        Term::Fun(_, args) => {
            for x in args {
                if contains_var(x, v) { return true }
            }
            false
        },
    }
}
