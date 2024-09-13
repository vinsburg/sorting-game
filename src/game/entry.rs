use crate::game::stack::kind::Kind;

#[derive(Clone)]
pub struct Entry {
    pub from: usize,
    pub to: usize,
    pub _kind: Kind,
    pub quantity: usize,
}
