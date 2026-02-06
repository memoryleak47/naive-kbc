use crate::*;

pub fn canonize_vars_l(e: Equation) -> Equation {
    canonize_vars_generic(e, |i| {
        let c = format!("L{i}");
        gsymb_add(c)
    })
}

pub fn canonize_vars_r(e: Equation) -> Equation {
    canonize_vars_generic(e, |i| {
        let c = format!("R{i}");
        gsymb_add(c)
    })
}

pub fn canonize_vars(e: Equation) -> Equation {
    canonize_vars_generic(e, |i| {
        let l = &["X", "Y", "W", "V", "U", "T", "S"];
        let a = l[i%7];
        let b = i/7;
        let b = if b == 0 { String::new() } else { format!("{}", b+1) };
        let c = format!("{a}{b}");
        gsymb_add(c)
    })
}

pub fn canonize_vars_generic((l, r, ori): Equation, name_fn: impl Fn(usize) -> Symbol) -> Equation {
    let mut v: Vec<Symbol> = Vec::new();
    acc_var_order(&l, &mut v);
    acc_var_order(&r, &mut v);

    let mut subst = Subst::new();
    for (i, x) in v.iter().enumerate() {
        let c = name_fn(i);
        subst.insert(*x, Term::Var(c));
    }
    let (l, r) = (apply_subst(&l, &subst), apply_subst(&r, &subst));

    (l, r, ori)
}

fn acc_var_order(t: &Term, acc: &mut Vec<Symbol>) {
    match t {
        Term::Var(v) => {
            if !acc.contains(v) {
                acc.push(*v);
            }
        }
        Term::Fun(_, children) => {
            for x in children.iter() {
                acc_var_order(x, acc);
            }
        },
    }
}

