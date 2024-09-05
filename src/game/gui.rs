use crate::game::Game;
use std::io::{self, Write};

impl Game {
    pub fn render(&self) {
        // Clear the screen and move the cursor to the top-left corner
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap(); // Ensure the screen is cleared immediately
        println!("|**************|\n| Sorting Game |\n****************");
        println!("{}", self.stage_name); // Display the current game name
        println!("Turn - {}", self.turn);
        println!();

        for (stack_ind, stack) in self.stacks.iter().enumerate() {
            let mut buffer: String = "".to_string();
            for unit in &stack.units {
                let unit_index = self.kind_indices[unit];
                let color = COLORS[unit_index % COLORS.len()].clone();
                buffer.push_str(
                    format!(
                        "\x1b[38;2;{};{};{}m{:>2}\x1b[0m ",
                        color[0],
                        color[1],
                        color[2],
                        unit.get_id()
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

    pub fn stage_complete_prompt(&self, is_last_stage: bool) {
        let game_complete_message = if is_last_stage { "You Won! 🎉" } else { "Stage complete! 💪" };
        println!("All Stacks Sorted! - {}\nPress Enter to continue, or Ctrl+C to exit.", game_complete_message);
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn read_valid_input(&self) -> (usize, usize) {
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
}

#[allow(dead_code)]
const COLORS: [[u8; 3]; 11] = [
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [255, 255, 0],
    [0, 255, 255],
    [255, 0, 255],
    // [127, 255, 0],
    // [0, 127, 255],
    // [255, 0, 127],
    [255, 127, 0],
    [0, 255, 127],
    // [127, 0, 255],
    [255, 127, 127],
    // [127, 255, 127],
    // [127, 127, 255],
    [127, 127, 127],
    [255, 255, 255],
    // [0, 0, 0],
];
