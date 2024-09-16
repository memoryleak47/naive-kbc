use std::collections::BTreeMap;
use std::sync::*;

// global symbol map.
static GSYMB: LazyLock<Mutex<SymbolMap>> = LazyLock::new(|| Mutex::from(SymbolMap::new()));

pub fn gsymb_add(x: String) -> Id {
    let mut g = GSYMB.lock().unwrap();
    g.add(x)
}

pub fn gsymb_get(x: Id) -> String {
    let g = GSYMB.lock().unwrap();
    g.get(x).to_string()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Id(pub usize);

// implementation of symbol map.

struct SymbolMap {
    string_to_id: BTreeMap<String, Id>,
    id_to_string: Vec<String>,
}

impl SymbolMap {
    fn new() -> Self {
        Self {
            string_to_id: Default::default(),
            id_to_string: Default::default(),
        }
    }

    fn add(&mut self, x: String) -> Id {
        if let Some(y) = self.string_to_id.get(&x) {
            return *y;
        } else {
            let i = self.string_to_id.len();
            self.string_to_id.insert(x.clone(), Id(i));
            self.id_to_string.push(x);
            Id(i)
        }
    }

    fn get(&self, id: Id) -> &str {
        self.id_to_string.get(id.0).unwrap()
    }
}


use std::cmp::*;

impl PartialOrd for Id {
    fn partial_cmp(&self, other: &Id) -> Option<Ordering> {
        let a = self;
        let b = other;

        let a = gsymb_get(*a);
        let b = gsymb_get(*b);

        let o = a.len().cmp(&b.len());
        if o != Ordering::Equal {
            return Some(o);
        }

        for (ca, cb) in a.chars().zip(b.chars()) {
            let o = ca.cmp(&cb);
            if o != Ordering::Equal { return Some(o); }
        }

        Some(Ordering::Equal)
    }
}

impl Ord for Id {
    fn cmp(&self, other: &Id) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
