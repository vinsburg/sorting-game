pub mod kind;

use std::iter::FlatMap;
use std::slice::Iter;
use std::vec::IntoIter;

use kind::{Kind, KindId};

pub struct Stack {
    capacity: usize,
    occupancy: usize,
    units: Vec<Kind>,
}

impl Stack {
    pub fn new(capacity: usize, occupancy: usize, units: Vec<Kind>) -> Stack {
        Stack {
            capacity,
            occupancy,
            units,
        }
    }

    pub fn clone(&self) -> Stack {
        Stack::new(
            self.get_capacity(),
            self.get_occupancy(),
            self.units.iter().map(|unit| unit.clone()).collect(),
        )
    }

    pub fn new_from_vec(vec: Vec<usize>) -> Stack {
        let mut units: Vec<Kind> = Vec::new();
        let mut kind: Kind;
        let capacity: usize = vec.len();
        let mut last_unit_id: KindId = Kind::get_empty_id();
        let mut quantity: usize = 0;
        let mut occupancy: usize = 0;
        for (ind, unit_id) in vec.iter().enumerate() {
            if (*unit_id != last_unit_id) || (ind == capacity - 1) {
                if last_unit_id != Kind::get_empty_id() {
                    kind = Kind::new(last_unit_id, quantity);
                    units.push(kind);
                    occupancy += quantity;
                }
                last_unit_id = *unit_id;
                quantity = 0;
            }
            quantity += 1;
        }
        // Push the last kind if it hasn't been pushed yet
        if last_unit_id != Kind::get_empty_id() {
            kind = Kind::new(last_unit_id, quantity);
            units.push(kind);
            occupancy += quantity;
        }

        Stack::new(capacity, occupancy, units)
    }

    pub fn is_vacant(&self) -> bool {
        self.get_occupancy() == 0
    }

    pub fn get_vacancy(&self) -> usize {
        self.get_capacity() - self.get_occupancy()
    }

    pub fn clone_top_unit(&self) -> Kind {  // TODO: use match expression instead.
        if self.get_occupancy() == 0 {
            return Kind::new_empty();
        } else {
            return self.units.last().unwrap().clone();
        }
    }

    pub fn get_top_unit_id(&self) -> KindId {
        match self.units.last() {
            Some(top_resident) => top_resident.get_id(),
            None => Kind::get_empty_id(),
        }
    }

    pub fn pop_residents_with_limit(&mut self, limit_: Option<usize>) -> Kind {
        let top_resident = &mut self.units.last_mut();
        let quantity: usize;

        match top_resident {
            Some(top_resident) => {
                quantity = limit_.unwrap_or_else(|| top_resident.get_quantity());

                let immigrants: Kind = if quantity < top_resident.get_quantity() {
                    let new_quantity = top_resident.get_quantity() - quantity;
                    top_resident.set_quantity(new_quantity);
                    Kind::new(top_resident.get_id(), quantity)
                } else {
                    self.units.pop().unwrap()
                };

                self.occupancy = self.occupancy.saturating_sub(immigrants.get_quantity());

                return immigrants;
            }
            None => return Kind::new_empty(),
        }
    }

    pub fn pop_residents(&mut self) -> Kind {
        self.pop_residents_with_limit(None)
    }

    pub fn push_immigrants(&mut self, immigrants: Kind) {
        let last_resident = self.units.last_mut();

        match last_resident {
            Some(top_resident) if top_resident.get_id() == immigrants.get_id() => {
                top_resident.set_quantity(top_resident.get_quantity() + immigrants.get_quantity());
            }
            _ => {
                self.units.push(immigrants);
            }
        }

        self.occupancy += immigrants.get_quantity();
    }

    pub fn iter_unit_ids(
        &self,
    ) -> FlatMap<Iter<'_, Kind>, IntoIter<KindId>, fn(&Kind) -> IntoIter<KindId>> {
        fn unit_to_ids(unit: &Kind) -> IntoIter<KindId> {
            vec![unit.get_id(); unit.get_quantity()].into_iter()
        }

        self.units.iter().flat_map(unit_to_ids)
    }

    pub fn get_occupancy(&self) -> usize {
        self.occupancy
    }

    pub fn get_capacity(&self) -> usize {
        self.capacity
    }
}
