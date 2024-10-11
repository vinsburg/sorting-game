mod entry;
mod gui;
mod stack;
mod stages;

use entry::Entry;
use stack::kind::{HasId, IsEmpty, Kind, KindId};
use stack::Stack;
use std::collections::HashMap;

pub struct Game {
    stacks: Vec<Stack>,
    units_per_kind: HashMap<KindId, usize>,
    kind_indices: HashMap<KindId, usize>,
    kinds_status: usize,
    turn: usize,
    stage_name: String,
    ledger: Vec<Entry>,
}

impl Game {
    fn new(stacks: Vec<Stack>, stage_name: Option<String>) -> Game {
        let units_per_kind: HashMap<KindId, usize> = Game::count_kinds(&stacks);
        let kind_indices: HashMap<KindId, usize> = Game::index_kinds(&units_per_kind);
        Game {
            stacks,
            units_per_kind,
            kind_indices,
            kinds_status: 0,
            turn: 1,
            stage_name: stage_name.unwrap_or("".to_string()),
            ledger: Vec::new(),
        }
    }

    fn clone(&self) -> Game {
        Game::new(
            self.stacks.iter().map(|stack| stack.clone()).collect(),
            Some(self.stage_name.clone()),
        )
    }

    fn count_kinds(stacks: &[Stack]) -> HashMap<KindId, usize> {
        let mut units_per_kind: HashMap<KindId, usize> = HashMap::new(); // Initialize the HashMap
        for stack in stacks {
            for unit_id in stack.iter_unit_ids() {
                *units_per_kind.entry(unit_id).or_insert(0) += 1; // Populate the HashMap
            }
        }
        units_per_kind
    }

    fn index_kinds(units_per_kind: &HashMap<KindId, usize>) -> HashMap<KindId, usize> {
        let mut kind_indices: HashMap<KindId, usize> = HashMap::new();
        let mut kind_ids: Vec<&KindId> = units_per_kind.keys().collect();
        kind_ids.sort(); // Sort kinds by their id
        for (index, kind) in kind_ids.iter().enumerate() {
            kind_indices.insert(**kind, index);
        }
        kind_indices
    }

    fn move_is_illegal(&self, from: usize, to: usize) -> bool {
        (from == to) || self.move_requires_more_room(from, to) || self.stack_tops_mismatch(from, to)
    }

    fn move_is_legal(&self, from: usize, to: usize) -> bool {
        !self.move_is_illegal(from, to)
    }

    fn move_requires_more_room(&self, from: usize, to: usize) -> bool {
        self.stacks[to].get_vacancy() < self.stacks[from].get_top_unit_quantity()
    }

    fn stack_tops_mismatch(&self, from: usize, to: usize) -> bool {
        let immigrant_id: KindId = self.stacks[from].get_top_unit_id();
        let resident_id: KindId = self.stacks[to].get_top_unit_id();

        let tops_match: bool =
            (immigrant_id == resident_id) || immigrant_id.is_empty() || resident_id.is_empty();
        !tops_match
    }

    fn no_legal_moves(&self) -> bool {
        for (from, _) in self.stacks.iter().enumerate() {
            for (to, _) in self.stacks.iter().enumerate() {
                if self.move_is_legal(from, to) {
                    return false;
                }
            }
        }
        true
    }

    fn update_kind_status(&mut self, stack_ind: usize) {
        let resident_id: KindId = self.stacks[stack_ind].get_top_unit_id();
        if resident_id.is_empty() {
            return;
        }
        let resident_quantity: usize = self.stacks[stack_ind].get_top_unit_quantity();

        let resident_bit: usize = 1 << self.get_kind_index(resident_id);
        self.kinds_status |= resident_bit; // Initially set the resident bit to 1.
        if resident_quantity != self.get_total_quantity(resident_id) {
            self.kinds_status -= resident_bit; // zero the resident bit.
        }
    }

    fn get_kind_index<T: HasId>(&self, kind_or_id: T) -> usize {
        self.kind_indices[&kind_or_id.get_id()]
    }

    fn get_total_quantity<T: HasId>(&self, kind_or_id: T) -> usize {
        self.units_per_kind[&kind_or_id.get_id()]
    }

    fn ledge(&mut self, from: usize, to: usize, kind: Kind, quantity: usize) {
        self.ledger.push(Entry {
            from,
            to,
            _kind: kind,
            quantity,
        });
    }

    fn update_state(&mut self, from: usize, to: usize) {
        self.update_kind_status(from);
        self.update_kind_status(to);
        self.turn += if self.stage_complete() { 0 } else { 1 };
    }

    fn move_units(&mut self, from: usize, to: usize, limit_: Option<usize>) {
        let kind: Kind = self.stacks[from].pop_residents_with_limit(limit_);
        let quantity: usize = kind.get_quantity();
        self.stacks[to].push_immigrants(kind);

        self.update_state(from, to);
        match limit_ {
            Some(_) => {} // Limits are specified in undo moves, Undo moves should not be ledged.
            _ => self.ledge(from, to, kind, quantity),
        };
    }

    fn move_legally(&mut self, from: usize, to: usize) {
        self.move_units(from, to, None);
    }

    fn move_forcefully(&mut self, from: usize, to: usize, quantity: usize) {
        self.move_units(from, to, Some(quantity));
    }

    fn stage_complete(&self) -> bool {
        self.kinds_status == (1 << self.units_per_kind.len()) - 1
    }

    fn undo_move(&mut self) {
        match self.ledger.last() {
            Some(entry) => {
                let (from, to, quantity) = (entry.to, entry.from, entry.quantity);
                self.ledger.pop();
                self.move_forcefully(from, to, quantity);
            }
            _ => {} // No moves to undo.
        }
    }

    fn turn_loop(&mut self) {
        let stage_backup: Game = self.clone();
        loop {
            if self.stage_complete() {
                break;
            }
            let user_input: gui::UserInput = self.read_valid_input();
            match user_input.stack_move {
                Some((from, to)) => self.move_legally(from, to),
                _ => match user_input.menu_option {
                    gui::MenuOption::Help => self.show_help(),
                    gui::MenuOption::Quit => std::process::exit(0),
                    gui::MenuOption::Reset => *self = stage_backup.clone(),
                    gui::MenuOption::Undo => self.undo_move(),
                    _ => {}
                },
            }
        }
    }

    pub fn play() {
        let stages: Vec<Game> = Game::get_stages();
        let last_stage_index: usize = stages.len() - 1;
        for (ind, mut stage) in stages.into_iter().enumerate() {
            stage.turn_loop();
            stage.stage_complete_prompt(ind == last_stage_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_stages() {
        let stages: Vec<Game> = Game::get_stages();
        let last_stage_index: usize = stages.len() - 1;

        assert!(last_stage_index > 0);
        let mut last_stage = stages[last_stage_index].clone();
        last_stage.move_legally(0, 1);
    }
}