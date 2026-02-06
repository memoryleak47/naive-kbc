use crate::*;

pub fn deduce_step(mut state: State) -> State {
    let mut cps = Vec::new();
    for l@(_, _, lori) in &state {
        if !lori { continue }
        for r@(_, _, rori) in &state {
            if !rori { continue }
            let (la, lb, _) = canonize_vars_l(l.clone());
            let (ra, rb, _) = canonize_vars_r(r.clone());

            for p in positions(&ra) {
                let Some(sig) = unify(&la, pos_idx(&ra, &p)) else { continue };
                let ll = apply_subst(&pos_set(&ra, &p, &lb), &sig);
                let rr = apply_subst(&rb, &sig);
                // TODO normalize cps here
                cps.push((ll, rr, false));
            }
        }
    }
    state.extend(cps);
    state
}
