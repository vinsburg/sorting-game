const EMPTY_SLOT_VALUE: usize = 0;

#[derive(Clone, PartialEq, Eq, Copy, Hash, Ord, PartialOrd)]
pub struct Kind {
    id: usize,
    quantity: usize,
}

impl Kind {
    pub fn new(id: usize, quantity: usize) -> Kind {
        Kind { id, quantity }
    }

    pub fn new_empty() -> Kind {
        Kind {
            id: EMPTY_SLOT_VALUE, quantity: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id == EMPTY_SLOT_VALUE
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
