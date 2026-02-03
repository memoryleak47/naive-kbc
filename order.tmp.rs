use crate::*;

use std::cmp::*;

// When comparing two terms by KBO there's four possible outputs:
// - terms are equal (Ordering::Equal)
// - first one is bigger (Ordering::Greater)
// - second one is bigger (Ordering::Less)
// - terms are incomparable (None)

// (true, _) means that we've found a subst under which lhs > rhs.
// (_, true) means that we've found a subst under which rhs > lhs.
type CmpState = (bool, bool);

const VAR_WEIGHT: usize = 1;
fn sym_weight(_x: Symbol) -> usize { 1 }

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Term) -> Option<Ordering> {
        match cmp(self, other) {
            (true, true) => None,
            (false, true) => Some(Ordering::Less),
            (true, false) => Some(Ordering::Greater),
            (false, false) => Some(Ordering::Equal),
        }
    }
}

fn cmp(t1: &Term, t2: &Term) -> CmpState {
    let st = varweight_cmpstate(t1, t2);
    if st.0 || st.1 { return st; }
}

fn varweight_cmpstate(t1: &Term, t2: &Term) -> CmpState {
    let w1 = term_weight(t1);
    let w2 = term_weight(t2);
    // The substitution mapping all vars to the constant "c" is completely decided by term weight.
    let (mut out1, mut out2) = (w1 > w2, w2 > w1);

    let vars1 = get_vars(t1);
    let vars2 = get_vars(t2);
    let vars = vars1.keys().chain(vars2.keys()).collect::<BTreeSet<_>>();

    for v in vars {
        // The substitution mapping all vars to the constant "c", but "v" to something arbitrarily large is used here!
        let c1 = vars1.get(v).unwrap_or(&0);
        let c2 = vars2.get(v).unwrap_or(&0);
        if c1 > c2 { out1 = true; }
        if c1 < c2 { out2 = true; }
    }
    (out1, out2)
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

fn term_weight(t: &Term) -> usize {
    match t {
        Term::Var(_) => VAR_WEIGHT,
        Term::Fun(x, args) => args.iter().map(term_weight).sum::<usize>() + sym_weight(*x),
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
