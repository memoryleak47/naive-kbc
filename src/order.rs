use crate::*;

use std::cmp::*;

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

    fn x() -> Term { Term::var("X") }
    fn y() -> Term { Term::var("Y") }
    fn c() -> Term { Term::cst("c") }
    fn c2() -> Term { Term::cst("c2") }
    fn d() -> Term { Term::cst("d") }
    fn fx() -> Term { Term::fun("f", [x()]) }
    fn fy() -> Term { Term::fun("f", [y()]) }
    fn fc() -> Term { Term::fun("f", [c()]) }
    fn fxy() -> Term { Term::fun("f", [x(), y()]) }
    fn fyx() -> Term { Term::fun("f", [y(), x()]) }

    #[track_caller]
    fn check_eq(l: Term, r: Term) {
        assert!(l <= r);
        assert!(r <= l);
        assert!(l >= r);
        assert!(r >= l);
    }

    #[track_caller]
    fn check_incompat(l: Term, r: Term) {
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
        check_eq(fxy(), fxy());
    }

    #[test]
    fn incompat_vars() {
        check_incompat(fx(), y());
        check_incompat(x(), y());
        check_incompat(fc(), x());
        check_incompat(c(), x());
    }

    #[test]
    fn weight_chk() {
        assert!(fx() > x());
        assert!(fc() > c());
        assert!(fx() > c());
    }

    #[test]
    fn lex_chk() {
        assert!(c() < d());
        assert!(c() < c2());
        assert!(c2() < d());
        check_incompat(fxy(), fyx());
    }
}
