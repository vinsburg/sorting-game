/*
Let's make a game where you have N stacks of size S, and K kinds where each type has U units, scattered across the stacks.
You may move units from stack s0 to stack p1 if the top stack units are of the same kind k0, and there is room on s1 for all k0 units from s0.
*/

use std::collections::HashSet;
use std::io::{self, Write}; // Import Write for flushing

const DEFAULT_STACK_SIZE: usize = 5;

#[derive(Clone, PartialEq, Eq, Copy)]
struct Kind {
    id: usize,
}

struct Stack {
    size: usize,
    units: Vec<Kind>,
}

#[allow(dead_code)]
impl Stack {
    fn new(units: Vec<Kind>) -> Stack {
        Stack {
            size: DEFAULT_STACK_SIZE,
            units,
        }
    }

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
    kinds_status: usize,
    turn: usize,
    colors: Vec<Vec<usize>>,
}

impl Game {
    fn new(stacks: Vec<Stack>) -> Game {
        let kinds_size = Game::count_kinds(&stacks);
        Game {
            stacks,
            kinds_size,
            kinds_status: 0,
            turn: 1,
            colors: vec![
                vec![255, 255, 255],
                vec![255, 0, 0],
                vec![0, 255, 0],
                vec![0, 0, 255],
                vec![255, 255, 0],
                vec![0, 255, 255],
                vec![255, 0, 255],
                // vec![127, 255, 0],
                vec![0, 127, 255],
                // vec![255, 0, 127],
                vec![255, 127, 127],
                vec![127, 255, 127],
                // vec![127, 127, 255],
                vec![0, 0, 0],
            ],
        }
    }

    fn count_kinds(stacks: &[Stack]) -> usize {
        let mut kinds: HashSet<usize> = HashSet::new();
        for stack in stacks {
            for unit in &stack.units {
                kinds.insert(unit.id);
            }
        }
        kinds.len()
    }

    fn render(&self) {
        // Clear the screen and move the cursor to the top-left corner
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap(); // Ensure the screen is cleared immediately
        for (stack_ind, stack) in self.stacks.iter().enumerate() {
            let mut buffer: String = "".to_string();
            for unit in &stack.units {
                let color = self.colors[unit.id % self.colors.len()].clone();
                buffer.push_str(
                    format!(
                        "\x1b[38;2;{};{};{}m{:>2}\x1b[0m ",
                        color[0], color[1], color[2], unit.id
                    )
                    .as_str(),
                );
            }
            for _ in stack.units.len()..stack.size {
                buffer.push_str("__ ");
            }
            println!("{:>2}: {}", stack_ind + 1, buffer);
        }
        println!();
    }

    fn make_a_move(&mut self, from: usize, to: usize) {
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
        if occupants.len() == self.stacks[to].size {
            self.kinds_status |= 1 << (from_top_occupant.id - 1);
        }
        self.stacks[to].push_occupants(occupants);
    }

    fn game_is_over(&self) -> bool {
        self.kinds_status == (1 << self.kinds_size) - 1
    }

    fn exit_if_player_won(&self) {
        if self.game_is_over() {
            println!("All Stacks Sorted! - You Won!");
            std::process::exit(0);
        }
    }

    fn read_valid_input(&self) -> (usize, usize) {
        let mut input = String::new();
        loop {
            print!("Select stacks to move from and to (e.g., '2 3'): ");
            io::stdout().flush().unwrap(); // Flush to ensure the message is displayed before reading input
            input.clear();
            io::stdin().read_line(&mut input).unwrap();

            let parts: Vec<&str> = input.trim().split_whitespace().collect();
            if parts.len() != 2 {
                println!("Please enter two numbers separated by a space.");
                continue;
            }

            let from = match parts[0].parse::<usize>() {
                Ok(num) if num - 1 < self.stacks.len() => num - 1,
                _ => {
                    println!(
                        "Invalid input for 'from' stack. Enter a number between 0 and {}.",
                        self.stacks.len() - 1
                    );
                    continue;
                }
            };

            let to = match parts[1].parse::<usize>() {
                Ok(num) if ((num - 1 < self.stacks.len()) && (num - 1 != from)) => num - 1,
                _ => {
                    println!(
                        "Invalid input for 'to' stack. Enter another number between 0 and {}.",
                        self.stacks.len() - 1
                    );
                    continue;
                }
            };

            return (from, to);
        }
    }

    pub fn turn_loop(&mut self) {
        loop {
            self.render();
            self.exit_if_player_won();
            println!("Turn {} -", self.turn);
            let (from, to) = self.read_valid_input();
            self.make_a_move(from, to);
        }
    }

    fn vecs_to_stacks(vecs: Vec<Vec<usize>>) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = Vec::new();
        for vec in vecs {
            let mut units: Vec<Kind> = Vec::new();
            for unit_id in vec {
                units.push(Kind { id: unit_id });
            }
            stacks.push(Stack::new(units));
        }
        stacks
    }

    pub fn game_0() -> Game {
        let vec_stacks = vec![
            vec![1, 2, 3],
            vec![5, 5, 3, 3, 4],
            vec![6, 7, 8, 2, 8],
            vec![9, 7, 7],
            vec![2, 7, 1, 10],
            vec![9, 5, 5, 3, 9],
            vec![7, 3, 10, 9],
            vec![],
            vec![6, 6, 1],
            vec![5, 8, 6],
            vec![8, 4, 9],
            vec![10, 10, 8, 6, 1],
            vec![2, 4, 1, 10],
            vec![4, 2, 4],
        ];
        let stacks = Game::vecs_to_stacks(vec_stacks);
        Game::new(stacks)
    }
}
