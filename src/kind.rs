const EMPTY_SLOT_VALUE: usize = 0;

#[derive(Clone, PartialEq, Eq, Copy, Hash, Ord, PartialOrd)]
pub struct Kind {
    id: usize,
}

impl Kind {
    pub fn new(id: usize) -> Kind {
        Kind { id }
    }

    pub fn new_empty() -> Kind {
        Kind {
            id: EMPTY_SLOT_VALUE,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id == EMPTY_SLOT_VALUE
    }

    pub fn get_id(&self) -> usize {
        self.id
    }
}
