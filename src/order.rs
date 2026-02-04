use crate::*;

// s > t
pub fn gt(s: &Term, t: &Term) -> bool {
    let vars_s = get_vars(s);
    let vars_t = get_vars(t);
    for (x, ct) in &vars_t {
        let cs = vars_s.get(x).unwrap_or(&0);
        // if t contains a variable more than s, then we have to return false.
        if ct > cs { return false }
    }

    let ws = weight(s);
    let wt = weight(t);
    if ws > wt { return true }
    if ws < wt { return false }

    assert_eq!(ws, wt);

    let Term::Fun(fs, ls) = s else { return false };
    let Term::Fun(ft, lt) = t else { return false };

    if fs > ft { return true }
    if fs < ft { return false }

    assert_eq!(fs, ft);

    for (cs, ct) in ls.iter().zip(lt.iter()) {
        if gt(cs, ct) { return true }

        if cs == ct { continue }
        else { return false }
    }

    assert!(s == t);

    false
}

fn weight(t: &Term) -> usize {
    match t {
        Term::Var(_) => 1,
        Term::Fun(_, children) => 1 + children.iter().map(weight).sum::<usize>(),
    }
}

fn get_vars(t: &Term) -> BTreeMap<Symbol, usize> {
    let mut out = BTreeMap::new();
    acc_vars(&t, &mut out);
    out
}

fn acc_vars(t: &Term, acc: &mut BTreeMap<Symbol, usize>) {
    match t {
        Term::Var(v) => {
            *acc.entry(*v).or_default() += 1;
        }
        Term::Fun(_, children) => {
            for x in children.iter() {
                acc_vars(x, acc);
            }
        },
    }
}


#[cfg(test)]
mod tst {
    use crate::*;

    fn kbo_assert(x: &str) {
        for op in ["~", "==", "<", ">"] {
            if x.contains(op) { // unrelated
                let [l, r] = *x.split(op).collect::<Vec<_>>() else { panic!() };
                let l = Term::parse(l).unwrap();
                let r = Term::parse(r).unwrap();

                let l_gt_r = gt(&l, &r);
                let r_gt_l = gt(&r, &l);

                match op {
                    "~" => { assert!(!l_gt_r); assert!(!r_gt_l); },
                    "==" => { assert!(l == r); assert!(!l_gt_r); assert!(!r_gt_l); },
                    "<" => { assert!(r_gt_l); assert!(!l_gt_r); },
                    ">" => { assert!(!r_gt_l); assert!(l_gt_r); },
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn refl() {
        kbo_assert("c == c");
        kbo_assert("X == X");
        kbo_assert("f(X) == f(X)");
        kbo_assert("f(X, Y) == f(X, Y)");
    }

    #[test]
    fn incompat_vars() {
        kbo_assert("f(X) ~ Y");
        kbo_assert("X ~ Y");
        kbo_assert("f(c) ~ X");
        kbo_assert("c ~ X");
    }

    #[test]
    fn weight_chk() {
        kbo_assert("f(X) > X");
        kbo_assert("f(c) > c");
        kbo_assert("f(X) > c");
    }

    #[test]
    fn lex_chk() {
        kbo_assert("c < d");
        kbo_assert("c < c2");
        kbo_assert("c2 < d");
        kbo_assert("f(X, Y) ~ f(Y, X)");
        kbo_assert("g(X) > f(a)");
    }

    #[test]
    fn assoc() {
        kbo_assert("f(f(X, Y), Z) > f(X, f(Y, Z))");
    }
}
