use crate::*;

pub type Equation = (/*lhs*/ Term, /*rhs*/ Term, /*is oriented*/ bool);

pub type State = Vec<Equation>;

pub fn kbc(mut state: State) -> State {
    for _ in 0..100 {
        state = kbc_step(state);
    }
    state
}

fn kbc_step(state: State) -> State {
    let state = nondeduce_step(state);
    let state = deduce_step(state);
    state
}

fn orient_one((l, r, ori): Equation) -> Equation {
    if ori { return (l, r, ori) }
    if gt(&l, &r) { return (l, r, ori) }
    if gt(&r, &l) { return (r, l, ori) }
    (l, r, ori)
}

fn simplify_one((l, r, ori): Equation, state: &State) -> Equation {
    let l2 = simplify(l.clone(), state);

    if ori {
        // TODO add |> ordering for rewrites.
    }
    let r2 = simplify(r.clone(), state);
    let ori2 = ori && (l == l2) && (r == r2);
    (l2, r2, ori2)
}

// TODO normalize & deduplicate rules
fn nondeduce_step(state: State) -> State {
    let mut new_state = Vec::new();
    for x in &state {
        // delete
        if x.0 == x.1 { continue }

        let x = x.clone();
        let x = orient_one(x);
        let x = simplify_one(x, &state);
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
