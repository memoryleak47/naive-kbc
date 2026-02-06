use crate::*;

pub type Pos = Vec<usize>;

pub fn positions(t: &Term) -> Vec<Pos> {
    match t {
        Term::Var(_) => vec![Vec::new()],
        Term::Fun(_, args) => {
            let mut out = vec![Vec::new()];
            for (i, a) in args.iter().enumerate() {
                for mut p in positions(a) {
                    p.insert(0, i);
                    out.push(p);
                }
            }
            out
        },
    }
}

// t[pos]
pub fn pos_idx<'t>(t: &'t Term, pos: &[usize]) -> &'t Term {
    match (t, pos.get(0)) {
        (t, None) => t,
        (Term::Fun(_, args), Some(i)) => pos_idx(&args[*i], &pos[1..]),
        _ => panic!(),
    }
}

// t[pos := x]
pub fn pos_set(t: &Term, pos: &[usize], x: &Term) -> Term {
    match (t, pos.get(0)) {
        (_, None) => x.clone(),
        (Term::Fun(f, args), Some(i)) => {
            let mut args = args.clone();
            args[*i] = pos_set(&args[*i], &pos[1..], x);
            Term::Fun(*f, args)
        }
        _ => panic!(),
    }
}
