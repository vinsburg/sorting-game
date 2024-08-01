const EMPTY_SLOT_VALUE: usize = 0;

#[derive(Clone, PartialEq, Eq, Copy, Hash, Ord, PartialOrd)]
pub struct Kind {
    pub id: usize,
}

impl Kind {
    pub fn is_empty(&self) -> bool {
        self.id == EMPTY_SLOT_VALUE
    }
}
