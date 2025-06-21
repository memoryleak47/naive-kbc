use crate::*;

use std::cmp::*;

// checks t1 < t2 of a Knuth-bendix ordering.
fn lt(t1: &Term, t2: &Term) -> bool {
    // make sure that no variable comes up more often in u than in v.
    let vars1 = get_vars(t1);
    let vars2 = get_vars(t2);
    for (v, c1) in &vars1 {
        let c2 = vars2.get(v).unwrap_or(&0);
        if c2 < c1 { return false; }
    }

    // weight check.
    let w1 = term_weight(t1);
    let w2 = term_weight(t2);
    if w1 < w2 { return true; }
    if w1 > w2 { return false; }

    match (t1, t2) {
        (Term::Fun(f1, children1), Term::Fun(f2, children2)) => {
            // lexicographic check:
            if f1 < f2 { return true; }
            if f1 > f2 { return false; }

            // recursion:
            for (c1, c2) in children1.iter().zip(children2.iter()) {
                if lt(c1, c2) { return true; }
                if c1 != c2 { return false; }
            }
            false
        },
        _ => false,
    }
}

const VAR_WEIGHT: usize = 1;
fn sym_weight(_x: Symbol) -> usize { 1 }

// When comparing two terms by KBO there's four possible outputs:
// - terms are equal (Ordering::Equal)
// - first one is bigger (Ordering::Greater)
// - second one is bigger (Ordering::Less)
// - terms are incomparable (None)

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        let vc = var_cmp(self, other)?;

        let mut out = weight_cmp(self, other);
        if out == Ordering::Equal {
            out = lex_cmp(self, other)?;
        }

        if vc == out || vc == Ordering::Equal {
            Some(out)
        } else {
            None
        }
    }
}

// returns None, if both terms have a majority in one variable.
fn var_cmp(l: &Term, r: &Term) -> Option<Ordering> {
    let lc = get_vars(l);
    let rc = get_vars(r);

    let mut out = Ordering::Equal;

    let vars = &lc.keys().collect::<BTreeSet<_>>() | &rc.keys().collect::<BTreeSet<_>>();
    for k in vars {
        let lv = lc.get(k).cloned().unwrap_or_default();
        let rv = rc.get(k).cloned().unwrap_or_default();
        out = merge_opt_orderings(out, lv.cmp(&rv))?;
    }

    Some(out)
}

fn merge_opt_orderings(a: Ordering, b: Ordering) -> Option<Ordering> {
    match (a, b) {
        (Ordering::Equal, y) => Some(y),
        (x, Ordering::Equal) => Some(x),
        (x, y) if x == y => Some(x),
        (_, _) => None,
    }
}

fn acc_vars(t: &Term, acc: &mut BTreeMap<Symbol, usize>) {
    match t {
        Term::Var(v) => {
            *acc.entry(*v).or_default() += 1;
        }
        Term::Fun(f, children) => {
            for x in children.iter() {
                acc_vars(x, acc);
            }
        },
    }
}

fn get_vars(t: &Term) -> BTreeMap<Symbol, usize> {
    let mut out = BTreeMap::new();
    acc_vars(&t, &mut out);
    out
}

// weight_cmp:
fn weight_cmp(l: &Term, r: &Term) -> Ordering {
    term_weight(l).cmp(&term_weight(r))
}

fn term_weight(t: &Term) -> usize {
    match t {
        Term::Var(_) => VAR_WEIGHT,
        Term::Fun(x, args) => args.iter().map(term_weight).sum::<usize>() + sym_weight(*x),
    }
}

// lex_cmp:
fn lex_cmp(l: &Term, r: &Term) -> Option<Ordering> {
    let mut lsyms = Vec::new();
    let mut rsyms = Vec::new();
    get_syms(l, &mut lsyms);
    get_syms(r, &mut rsyms);

    let ls = lsyms.iter().cloned();
    let rs = rsyms.iter().cloned();

    for ((lc, lvar), (rc, rvar)) in ls.zip(rs) {
        // comparing the variable X to anything other than X will always be incomparable.
        if (lvar || rvar) && (lc, lvar) != (rc, rvar) {
            return None;
        }

        let ord = lc.cmp(&rc);
        if ord != Ordering::Equal { return Some(ord); }
    }

    Some(Ordering::Equal)
}

// true = var, false = normal symbol.
fn get_syms(l: &Term, out: &mut Vec<(Symbol, bool)>) {
    match l {
        Term::Var(x) => out.push((*x, true)),
        Term::Fun(x, children) => {
            out.push((*x, false));

            for ch in children.iter() {
                get_syms(ch, out);
            }
        },
    }
}

mod tst {
    use crate::*;

    fn kbo_assert(x: &str) {
        for op in ["~", "<=", ">=", "==", "<", ">"] {
            if x.contains(op) { // unrelated
                let [l, r] = *x.split(op).collect::<Vec<_>>() else { panic!() };
                let l = Term::parse(l).unwrap();
                let r = Term::parse(r).unwrap();
                match op {
                    "~" => assert!(l.partial_cmp(&r).is_none()),
                    "<=" => assert!(l <= r),
                    ">=" => assert!(l >= r),
                    "==" => { assert!(l == r); assert!(l <= r); assert!(r <= l); },
                    "<" => assert!(l < r),
                    ">" => assert!(l > r),
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

    // TODO this should work! but we currently implement "fake KBO" instead of KBO. So this is why it fails.
    #[test]
    fn assoc() {
        kbo_assert("f(f(X, Y), Z) > f(X, f(Y, Z))");
    }
}
