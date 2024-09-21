pub mod kind;

use std::slice::Iter;

use kind::Kind;

pub struct Stack {
    capacity: usize,
    units: Vec<Kind>,
}

impl Stack {
    pub fn new(capacity: usize, units: Vec<Kind>) -> Stack {
        Stack { capacity, units }
    }

    pub fn new_empty() -> Stack {
        Stack::new(0, Vec::new())
    }

    pub fn clone(&self) -> Stack {
        Stack::new(
            self.capacity(),
            self.units.iter().map(|unit| unit.clone()).collect(),
        )
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

    pub fn pop_immigrants_with_limit(&mut self, immigrants: &mut Stack, limit_: Option<usize>) -> Kind{
        let top_immigrant: Kind = self.clone_top_unit();
        let mut next_immigrant_is_legal: bool = !self.is_vacant();
        let mut counter: usize = match limit_ {
            Some(q) => q,
            None => self.len(),
        };
        let kind_immigrant: Kind = Kind::new(top_immigrant.get_id(), counter);

        while next_immigrant_is_legal && (counter > 0) {
            self.units.pop();
            immigrants.units.push(top_immigrant.clone());
            counter -= 1;
            next_immigrant_is_legal = self.clone_top_unit() == top_immigrant;
        }

        kind_immigrant
    }

    pub fn pop_immigrants(&mut self, immigrants: &mut Stack) -> Kind {
        self.pop_immigrants_with_limit(immigrants, None)
    }

    pub fn push_immigrants(&mut self, immigrants: &mut Stack) {
        while immigrants.len() != 0 {
            let immigrant: Kind = immigrants.units.pop().unwrap();
            self.units.push(immigrant);
        }
    }

    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn iter_units(&self) -> Iter<'_, Kind> {
        self.units.iter()
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}
