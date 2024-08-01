use crate::Kind;

pub struct Stack {
    pub size: usize,
    pub units: Vec<Kind>,
}

impl Stack {
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

    pub fn pop_immigrants(&mut self, immigrants: &mut Stack) {
        let top_immigrant = self.clone_top_unit();
        while !self.is_vacant() && self.clone_top_unit() == top_immigrant {
            self.units.pop();
            immigrants.units.push(top_immigrant.clone());
        }
    }

    pub fn push_immigrants(&mut self, immigrants: &mut Stack) {
        while immigrants.units.len() != 0 {
            let immigrant = immigrants.units.pop().unwrap();
            self.units.push(immigrant);
        }
    }
}
