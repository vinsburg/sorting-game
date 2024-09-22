pub mod kind;

use std::iter::Map;
use std::slice::Iter;

use kind::{Kind, KindId};

pub struct Stack {
    capacity: usize,
    units: Vec<Kind>,
}

impl Stack {
    pub fn new(capacity: usize, units: Vec<Kind>) -> Stack {
        Stack { capacity, units }
    }

    pub fn clone(&self) -> Stack {
        Stack::new(
            self.capacity(),
            self.units.iter().map(|unit| unit.clone()).collect(),
        )
    }

    pub fn new_from_vec(vec: Vec<usize>) -> Stack {
        let mut units: Vec<Kind> = Vec::new();
        let mut kind: Kind;
        let capacity: usize = vec.len();
        for unit_id in vec {
            kind = Kind::new(unit_id, 1);
            if !kind.is_empty() {
                units.push(kind);
            }
        }
        Stack::new(capacity, units)
    }

    pub fn is_vacant(&self) -> bool {
        self.len() == 0
    }

    pub fn get_vacancy(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn clone_top_unit(&self) -> Kind {
        if self.len() == 0 {
            return Kind::new_empty();
        } else {
            return self.units.last().unwrap().clone();
        }
    }

    pub fn pop_immigrants_with_limit(&mut self, limit_: Option<usize>) -> Kind {
        let top_immigrant: Kind = self.clone_top_unit();
        let mut next_immigrant_is_legal: bool = !self.is_vacant();
        let limit: usize = match limit_ {
            Some(q) => q,
            None => self.len(),
        };
        let mut quantity: usize = 0;

        while next_immigrant_is_legal && (limit > quantity) {
            self.units.pop();
            quantity += 1;
            next_immigrant_is_legal = self.clone_top_unit() == top_immigrant;
        }

        Kind::new(top_immigrant.get_id(), quantity)
    }

    pub fn pop_immigrants(&mut self) -> Kind {
        self.pop_immigrants_with_limit(None)
    }

    pub fn push_immigrants(&mut self, immigrants: Kind) {
        let immigrant: Kind = Kind::new(immigrants.get_id(), 1);
        for _ in 0..immigrants.get_quantity() {
            self.units.push(immigrant.clone());
        }
    }

    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn iter_units(&self) -> Iter<'_, Kind> {
        self.units.iter()
    }

    pub fn iter_unit_ids(&self) -> Map<Iter<'_, Kind>, fn(&Kind) -> KindId> {
        self.iter_units().map(|unit| unit.get_id())
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
