/*
The sorting game is comprised of stacks of varying sizes, and several "kinds", each with multiple units, scattered across the stacks.
You may move units from stack s0 to stack s1 if the top stack units are of the same kind k0, and there is room on s1 for all k0 units from s0.
The goal is for all stacks to be either empty, or contain all units of a single kind.
*/

mod gui;
mod stack;
mod stages;

use stack::kind::Kind;
use stack::Stack;
use std::collections::HashMap;

pub struct Game {
    stacks: Vec<Stack>,
    units_per_kind: HashMap<Kind, usize>,
    kind_indices: HashMap<Kind, usize>,
    kinds_status: usize,
    turn: usize,
    stage_name: String,
}

impl Game {
    fn new(stacks: Vec<Stack>, stage_name: Option<String>) -> Game {
        let units_per_kind = Game::count_kinds(&stacks);
        let kind_indices = Game::index_kinds(&units_per_kind);
        Game {
            stacks,
            units_per_kind,
            kind_indices,
            kinds_status: 0,
            turn: 1,
            stage_name: stage_name.unwrap_or("".to_string()),
        }
    }

    fn clone(&self) -> Game {
        Game::new(
            self.stacks.iter().map(|stack| stack.clone()).collect(),
            Some(self.stage_name.clone()),
        )
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

    fn update_status(&mut self, stack_ind: usize) {
        let immigrants = &mut Stack::new();

        self.stacks[stack_ind].pop_immigrants(immigrants);
        let top_immigrant: Kind = immigrants.clone_top_unit();
        if !top_immigrant.is_empty()
            && (immigrants.units.len() == self.units_per_kind[&top_immigrant])
        {
            self.kinds_status |= 1 << self.kind_indices[&top_immigrant];
        } else {
            self.kinds_status &= !(1 << self.kind_indices[&top_immigrant]);
        }
        self.stacks[stack_ind].push_immigrants(immigrants);

        self.turn += if self.stage_complete() { 0 } else { 1 };
    }

    fn make_a_move(&mut self, from: usize, to: usize) {
        // TODO: implement forced illegal moves, for undo support.
        let immigrants: &mut Stack = &mut Stack::new();
        self.stacks[from].pop_immigrants(immigrants);

        let move_is_legal: bool = self.move_is_legal(&immigrants, &self.stacks[to]);
        let dest: usize = if move_is_legal { to } else { from };
        self.stacks[dest].push_immigrants(immigrants);

        self.update_status(to);
    }

    fn stage_complete(&self) -> bool {
        self.kinds_status == (1 << self.units_per_kind.len()) - 1
    }

    fn turn_loop(&mut self) {
        // TODO: Implement stage reset with some key combination.
        let stage_backup: Game = self.clone();
        loop {
            self.render();
            if self.stage_complete() {
                break;
            }
            let (from, to) = self.read_valid_input();
            if (from == 0) && (to == 0) {
                *self = stage_backup.clone();
                continue;
            }
            self.make_a_move(from, to);
        }
    }

    pub fn play() {
        let stages = Game::get_stages();
        let last_stage_index = stages.len() - 1;
        for (ind, mut stage) in stages.into_iter().enumerate() {
            stage.turn_loop();
            stage.stage_complete_prompt(ind == last_stage_index);
        }
    }
}
