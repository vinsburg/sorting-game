use crate::game::stack::kind::Kind;

#[derive(Clone)]
pub struct Entry {
    pub _from: usize,
    pub _to: usize,
    pub _kind: Kind,
    pub _quantity: usize,
}
