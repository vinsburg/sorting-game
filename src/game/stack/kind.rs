const EMPTY_SLOT_VALUE: KindId = 0;

pub type KindId = usize;

pub trait HasId {
    fn get_id(&self) -> KindId;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl HasId for KindId {
    fn get_id(&self) -> KindId {
        *self
    }
}

impl IsEmpty for KindId {
    fn is_empty(&self) -> bool {
        *self == Kind::get_empty_id()
    }
}

#[derive(Clone, PartialEq, Eq, Copy, Hash, Ord, PartialOrd)]
pub struct Kind {
    id: KindId,
    quantity: usize,
}

impl Kind {
    pub fn new(id: usize, quantity: usize) -> Kind {
        Kind { id, quantity }
    }

    pub fn new_empty() -> Kind {
        Kind {
            id: Kind::get_empty_id(),
            quantity: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.id == Kind::get_empty_id()
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_quantity(&self) -> usize {
        self.quantity
    }

    pub fn set_quantity(&mut self, quantity: usize) {
        self.quantity = quantity;
    }

    pub fn get_empty_id() -> KindId {
        EMPTY_SLOT_VALUE
    }
}

impl HasId for Kind {
    fn get_id(&self) -> KindId {
        self.id
    }
}
