/*
Let's make a game where you have N stacks of size S, and K kinds where each type has U units, scattered across the stacks.
You may move units from stack s0 to stack p1 if the top stack units are of the same kind k0, and there is room on s1 for all k0 units from s0.
*/
use std::fmt;

#[derive(Clone, PartialEq, Eq, Copy)]
struct Kind {
    id: usize,
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The debug output of kind is the kind's id.
        write!(f, "{}", self.id)
    }
}

struct Stack {
    size: usize,
    units: Vec<Kind>,
}

#[allow(dead_code)]
impl Stack {
    fn is_vacant(&self) -> bool {
        self.units.len() == 0
    }

    fn get_vacancy(&self) -> usize {
        self.size - self.units.len()
    }

    fn get_top_occupant_kind(&self) -> Kind {
        if self.units.len() == 0 {
            return Kind { id: 0 };
        } else {
            return self.units.last().unwrap().clone();
        }
    }

    fn pop_top_occupants(&mut self, occupants: &mut Vec<Kind>) {
        let top_occupant = self.get_top_occupant_kind();
        while !self.is_vacant() && self.get_top_occupant_kind() == top_occupant {
            self.units.pop();
            occupants.push(top_occupant.clone());
        }
    }

    fn push_occupants(&mut self, occupants: &mut Vec<Kind>) {
        while occupants.len() != 0 {
            let occupant = occupants.pop().unwrap();
            self.units.push(occupant);
        }
    }
}

pub struct Game {
    stacks: Vec<Stack>,
    kinds_size: usize,
    turn: usize,
}

impl Game {
    pub fn new(
        stack_quantity: usize,
        stack_size: usize,
        units_per_kind: usize,
        kinds_size: usize,
        seed: u32,
    ) -> Game {
        println!("Seed: {}", seed); // TODO: use seed to randomize the game.
                                    // let mut kinds = Vec::new();
        let mut units = Vec::new();
        for id in 1..kinds_size + 1 {
            // kinds.push(Kind { id: id });
            for _ in 0..units_per_kind {
                units.push(Kind { id: id });
            }
        }

        let units_per_stack: usize = (kinds_size * units_per_kind) / stack_quantity;
        let mut stacks = Vec::new();
        for _ in 0..stack_quantity {
            let mut stack_units = Vec::new();
            for _ in 0..units_per_stack {
                let unit = units.pop().unwrap();
                stack_units.push(unit);
            }
            stacks.push(Stack {
                size: stack_size,
                // Init a vec of type Unit:
                units: stack_units,
            });
        }

        let turn = 1;

        Game {
            stacks,
            kinds_size,
            turn,
        } //, kinds }
    }

    pub fn render(&self) {
        println!();
        for (stack_ind, stack) in self.stacks.iter().enumerate() {
            let mut render_vec: Vec<String> = Vec::new();
            for unit in &stack.units {
                render_vec.push(format!("{:?}", unit));
            }
            for _ in stack.units.len()..stack.size {
                render_vec.push("_".to_string());
            }
            println!("{}: {:?}", stack_ind, render_vec);
        }
        println!();
    }

    pub fn make_a_move(&mut self, from: usize, to: usize) {
        self.turn += 1;

        let occupants = &mut Vec::new();
        let from_top_occupant = self.stacks[from].get_top_occupant_kind();
        self.stacks[from].pop_top_occupants(occupants);

        if !self.stacks[to].is_vacant() {
            let to_top_occupant = self.stacks[to].get_top_occupant_kind();
            let to_vacancy: usize = self.stacks[to].get_vacancy();
            if (from_top_occupant != to_top_occupant) || (to_vacancy < occupants.len()) {
                self.stacks[from].push_occupants(occupants);
            }
        }

        if occupants.len() != 0 {
            self.stacks[to].push_occupants(occupants);
        }

        self.stacks[to].pop_top_occupants(occupants);
        if occupants.len() < self.stacks[to].size {
            self.stacks[to].push_occupants(occupants);
        } else {
            println!("\nKind {:?} cleared!", from_top_occupant);
            self.kinds_size -= 1;
        }
    }

    pub fn game_is_over(&self) -> bool {
        self.kinds_size == 0
    }

    pub fn exit_if_player_won(&self) {
        if self.game_is_over() {
            println!("All Kinds Cleared! - You won!");
            std::process::exit(0);
        }
    }

    pub fn turn_loop(&mut self) {
        loop {
            self.render();
            self.exit_if_player_won();
            let mut from = String::new();
            let mut to = String::new();
            println!("Turn {} -", self.turn);
            println!("Select stack to move from:");
            std::io::stdin().read_line(&mut from).unwrap();
            println!("Select stack to move to:");
            std::io::stdin().read_line(&mut to).unwrap();
            let from: usize = from.trim().parse().unwrap();
            let to: usize = to.trim().parse().unwrap();
            self.make_a_move(from, to);
        }
    }
}
