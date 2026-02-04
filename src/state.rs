use crate::*;

pub type Equation = (/*lhs*/ Term, /*rhs*/ Term, /*is oriented*/ bool);

pub type State = Vec<Equation>;

pub fn kbc(mut state: State) -> State {
    for _ in 0..100 {
        state = kbc_step(state);
    }
    state
}

pub fn kbc_step(state: State) -> State {
    let state = nondeduce_step(state);
    let state = deduce_step(state);
    state
}

fn orient_one((l, r, ori): Equation) -> Equation {
    if ori { return (l, r, ori) }
    if gt(&l, &r) { return (l, r, true) }
    if gt(&r, &l) { return (r, l, true) }
    (l, r, ori)
}

pub fn simplify(mut rw: Equation, state: &State) -> Equation {
    for rw_@(l_, r_, ori_) in state {
        if !ori_ { continue }

        let (l, r, ori) = &rw;

        // output:
        let l2 = if !ori || ruleorder_gt(&rw, &rw_) {
            simplify_single(r.clone(), &rw_)
        } else { l.clone() };

        let r2 = simplify_single(r.clone(), &rw_);

        let ori2 = *ori && (*l == l2);

        rw = (l2, r2, ori2);
    }
    rw
}

// t > p, if a subterm of t is a substitution instance of p.
// in other words, if a rule with pattern "p" is somewhere applicable in "t".
fn encompassment_gt(t: &Term, p: &Term) -> bool {
    if pat_match(p, t).is_some() { return true }
    let Term::Fun(_f, args) = t else { return false };
    for x in args {
        if encompassment_gt(x, p) { return true }
    }
    false
}

// s -> t |> l -> r
fn ruleorder_gt((s, t, _): &Equation, (l, r, _): &Equation) -> bool {
    let out = encompassment_gt(s, l) || (/*this should be a "literally similar" check*/ s == l && gt(t, r));
    out
}

// TODO normalize & deduplicate rules
fn nondeduce_step(state: State) -> State {
    let mut new_state = Vec::new();
    for x in &state {
        // delete
        if x.0 == x.1 { continue }

        let x = x.clone();
        let x = orient_one(x);
        let x = simplify(x, &state);
        new_state.push(x);
    }

    new_state
}

fn deduce_step(state: State) -> State {
    state // TODO
}

pub fn dump_state(state: &State) {
    println!("STATE:");
    for (l, r, ori) in state {
        let l = l.to_string();
        let r = r.to_string();
        let op = if *ori { "->" } else { "=" };
        println!("{l} {op} {r}");
    }
}
