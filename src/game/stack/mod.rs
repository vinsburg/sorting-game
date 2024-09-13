pub mod kind;

use kind::Kind;

pub struct Stack {
    pub size: usize,
    pub units: Vec<Kind>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            size: 0,
            units: Vec::new(),
        }
    }

    pub fn clone(&self) -> Stack {
        Stack {
            size: self.size,
            units: self.units.iter().map(|unit| unit.clone()).collect(),
        }
    }

    pub fn is_vacant(&self) -> bool {
        self.units.len() == 0
    }

    pub fn get_vacancy(&self) -> usize {
        self.size - self.units.len()
    }

    pub fn clone_top_unit(&self) -> Kind {
        if self.units.len() == 0 {
            return Kind::new_empty();
        } else {
            return self.units.last().unwrap().clone();
        }
    }

    pub fn pop_immigrants_with_limit(&mut self, immigrants: &mut Stack, limit_: Option<usize>) {
        let top_immigrant: Kind = self.clone_top_unit();
        let mut next_immigrant_is_legal: bool = true;
        let mut counter: usize = match limit_ {
            Some(q) => q,
            None => self.units.len(),
        };

        while !self.is_vacant() && (counter > 0) && next_immigrant_is_legal {
            self.units.pop();
            immigrants.units.push(top_immigrant.clone());
            counter -= 1;
            next_immigrant_is_legal = self.clone_top_unit() == top_immigrant;
        }
    }

    pub fn pop_immigrants(&mut self, immigrants: &mut Stack) {
        self.pop_immigrants_with_limit(immigrants, None);
    }

    pub fn push_immigrants(&mut self, immigrants: &mut Stack) {
        while immigrants.units.len() != 0 {
            let immigrant: Kind = immigrants.units.pop().unwrap();
            self.units.push(immigrant);
        }
    }
}
