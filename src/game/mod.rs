/*
The sorting game is comprised of stacks of varying sizes, and several "kinds", each with multiple units, scattered across the stacks.
You may move units from stack s0 to stack s1 if the top stack units are of the same kind k0, and there is room on s1 for all k0 units from s0.
The goal is for all stacks to be either empty, or contain all units of a single kind.
*/

mod gui;
mod stages;
mod stack;

use stack::kind::Kind;
use stack::Stack;
use std::collections::HashMap;

pub struct Game {
    stacks: Vec<Stack>,
    units_per_kind: HashMap<Kind, usize>,
    kind_indices: HashMap<Kind, usize>,
    kinds_status: usize,
    turn: usize,
    name: String,
}

impl Game {
    fn new(stacks: Vec<Stack>) -> Game {
        let units_per_kind = Game::count_kinds(&stacks);
        let kind_indices = Game::index_kinds(&units_per_kind);
        Game {
            stacks,
            units_per_kind,
            kind_indices,
            kinds_status: 0,
            turn: 1,
            name: "".to_string(),
        }
    }

    fn count_kinds(stacks: &[Stack]) -> HashMap<Kind, usize> {
        let mut units_per_kind: HashMap<Kind, usize> = HashMap::new(); // Initialize the HashMap
        for stack in stacks {
            for unit in &stack.units {
                *units_per_kind.entry(*unit).or_insert(0) += 1; // Populate the HashMap
            }
        }
        units_per_kind
    }

    fn index_kinds(units_per_kind: &HashMap<Kind, usize>) -> HashMap<Kind, usize> {
        let mut kind_indices: HashMap<Kind, usize> = HashMap::new();
        let mut kinds: Vec<&Kind> = units_per_kind.keys().collect();
        kinds.sort(); // Sort kinds by their id
        for (index, kind) in kinds.iter().enumerate() {
            kind_indices.insert(**kind, index);
        }
        kind_indices
    }

    fn move_is_legal(&self, immigrants: &Stack, residents: &Stack) -> bool {
        let top_immigrant = immigrants.clone_top_unit();
        let top_resident = residents.clone_top_unit();
        let tops_match =
            (top_immigrant == top_resident) || top_immigrant.is_empty() || top_resident.is_empty();
        let there_is_room = immigrants.units.len() <= residents.get_vacancy();
        tops_match && there_is_room
    }

    fn make_a_move(&mut self, from: usize, to: usize) {
        self.turn += 1;

        let immigrants = &mut Stack {
            size: 0,
            units: Vec::new(),
        };
        self.stacks[from].pop_immigrants(immigrants);

        let move_is_legal = self.move_is_legal(&immigrants, &self.stacks[to]);

        if move_is_legal {
            self.stacks[to].push_immigrants(immigrants);
        } else {
            self.stacks[from].push_immigrants(immigrants);
        }

        self.stacks[to].pop_immigrants(immigrants);
        let top_immigrant: Kind = immigrants.clone_top_unit();
        if !top_immigrant.is_empty()
            && (immigrants.units.len() == self.units_per_kind[&top_immigrant])
        {
            self.kinds_status |= 1 << self.kind_indices[&top_immigrant];
        }
        self.stacks[to].push_immigrants(immigrants);
    }

    fn game_is_over(&self) -> bool {
        self.kinds_status == (1 << self.units_per_kind.len()) - 1
    }

    pub fn turn_loop(&mut self) {
        loop {
            self.render();
            if self.display_game_end() {
                break;
            }
            println!("Turn {} -", self.turn);
            let (from, to) = self.read_valid_input();
            self.make_a_move(from, to);
        }
    }
}
