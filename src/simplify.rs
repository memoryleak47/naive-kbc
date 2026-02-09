use crate::*;

pub fn simplify_converge(eq: Equation, state: &State) -> Equation {
    // bring eq into special d normal form to differentiate it from other rules.
    let mut eq = canonize_vars_d(eq);

    loop {
        let eq2 = simplify(eq.clone(), state);
        if eq == eq2 { return eq }
        eq = eq2;
    }
}

pub fn simplify(mut rw: Equation, state: &State) -> Equation {
    for rw_@(_, _, ori_) in state {
        if !ori_ { continue }
        assert!(v_disjoint(&get_vars_eq(&rw), &get_vars_eq(&rw_)));

        let (l, r, ori) = &rw;

        // output:
        let l2 = if !ori || ruleorder_gt(&rw, &rw_) {
            simplify_single(l.clone(), &rw_)
        } else { l.clone() };

        let r2 = simplify_single(r.clone(), &rw_);

        let ori2 = *ori && (*l == l2);

        rw = (l2, r2, ori2);
    }
    rw
}

pub fn simplify_single(mut term: Term, eq: &Equation) -> Term {
    let (_, _, ori) = eq;
    assert!(ori);

    assert!(v_disjoint(&get_vars(&term), &get_vars_eq(&eq)));

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

// s -> t |> l -> r
fn ruleorder_gt((s, t, _): &Equation, (l, r, _): &Equation) -> bool {
    if literally_similar(&s, &l) {
        gt(t, r)
    } else {
        encompassment_gte(s, l)
    }
}

// t >= p, if a subterm of t is a substitution instance of p.
// in other words, if a rule with pattern "p" is somewhere applicable in "t".
fn encompassment_gte(t: &Term, p: &Term) -> bool {
    if pat_match(p, t).is_some() { return true }
    let Term::Fun(_f, args) = t else { return false };
    for x in args {
        if encompassment_gte(x, p) { return true }
    }
    false
}

