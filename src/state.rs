use crate::*;

pub type Equation = (/*lhs*/ Term, /*rhs*/ Term, /*is oriented*/ bool);

pub type State = Vec<Equation>;

pub fn kbc(mut state: State) -> State {
    dump_state(&state);
    loop {
        println!("-------------------");
        let state2 = kbc_step(state.clone());
        if state == state2 { return state }
        state = state2;
        dump_state(&state);
    }
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

// TODO normalize & deduplicate rules
fn nondeduce_step(state: State) -> State {
    let mut new_state = Vec::new();
    for x in &state {
        // delete
        if x.0 == x.1 { continue }

        let x = x.clone();
        let x = orient_one(x);
        let x = simplify_converge(x, &state);
        let x = canonize_vars(x);
        if !new_state.contains(&x) {
            new_state.push(x);
        }
    }

    new_state
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
