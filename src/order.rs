use crate::*;

use std::cmp::*;

const VAR_WEIGHT: usize = 2;
fn sym_weight(_x: Symbol) -> usize { 1 }

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

// var_cmp:
fn var_cmp(l: &Term, r: &Term) -> Option<Ordering> {
    let mut lc = BTreeMap::new();
    let mut rc = BTreeMap::new();
    get_vars(l, &mut lc);
    get_vars(r, &mut rc);

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

fn get_vars(t: &Term, acc: &mut BTreeMap<Symbol, usize>) {
    match t {
        Term::Var(v) => {
            *acc.entry(*v).or_default() += 1;
        }
        Term::Fun(f, children) => {
            for x in children.iter() {
                get_vars(x, acc);
            }
        },
    }
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

    fn x() -> Term { Term::var("X") }
    fn y() -> Term { Term::var("Y") }
    fn c() -> Term { Term::cst("c") }
    fn fx() -> Term { Term::fun("f", [x()]) }
    fn fy() -> Term { Term::fun("f", [y()]) }
    fn fc() -> Term { Term::fun("f", [c()]) }

    #[track_caller]
    fn check_eq(l: Term, r: Term) {
        assert!(l <= r);
        assert!(r <= l);
        assert!(l >= r);
        assert!(r >= l);
    }

    #[track_caller]
    fn check_neq(l: Term, r: Term) {
        assert!(!(l <= r));
        assert!(!(r <= l));
        assert!(!(l >= r));
        assert!(!(r >= l));
    }

    #[test]
    fn refl() {
        check_eq(c(), c());
        check_eq(x(), x());
        check_eq(fx(), fx());
    }

    #[test]
    fn incompat_vars() {
        check_neq(fx(), y());
        check_neq(x(), y());
    }

    #[test]
    fn weight_chk() {
        assert!(fx() > x());
        assert!(fc() > c());
    }
}
