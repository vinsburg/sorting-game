use crate::game::Game;
use std::io::{self, Write};

pub enum MenuOption {
    Help,
    Move,
    Reset,
    Undo,
    Quit,
}

pub struct UserInput {
    pub menu_option: MenuOption,
    pub stack_move: Option<(usize, usize)>,
}

impl UserInput {
    pub fn new_menu_option(menu_option: MenuOption) -> UserInput {
        UserInput {
            menu_option: menu_option,
            stack_move: None,
        }
    }
}

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
                let unit_index: usize = self.kind_indices[unit];
                let color: [u8; 3] = COLORS[unit_index % COLORS.len()].clone();
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
        let game_complete_message: &str = match is_last_stage {
            true => "You Won! 🎉",
            false => "Stage complete! 💪",
        };
        println!(
            "All Stacks Sorted! - {}\nPress Enter to continue, or Ctrl+C to exit.",
            game_complete_message
        );
        io::stdin().read_line(&mut String::new()).unwrap();
    }

    pub fn read_valid_input(&self) -> UserInput {
        let user_input: UserInput;
        let mut input: String = String::new();
        loop {
            print!("Select stacks to move from and to (e.g., '2 3').\nType 'u' to undo or 'r' to reset the stage: ");
            io::stdout().flush().unwrap(); // Flush to ensure the message is displayed before reading input
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let str_input: &str = input.trim();

            user_input = match str_input {
                "h" => UserInput::new_menu_option(MenuOption::Help),
                "r" => UserInput::new_menu_option(MenuOption::Reset),
                "u" => UserInput::new_menu_option(MenuOption::Undo),
                "q" => UserInput::new_menu_option(MenuOption::Quit),
                _ => {
                    let parts: Vec<&str> = input.trim().split_whitespace().collect();
                    if parts.len() != 2 {
                        println!("Please enter two numbers separated by a space.");
                        continue;
                    }

                    let from = match parts[0].parse::<usize>() {
                        Ok(num) if num - 1 < self.stacks.len() => num - 1,
                        _ => {
                            println!(
                                "Invalid 'from' stack. Enter a number between 0 and {}.",
                                self.stacks.len() - 1
                            );
                            continue;
                        }
                    };

                    let to = match parts[1].parse::<usize>() {
                        Ok(num) if ((num - 1 < self.stacks.len()) && (num - 1 != from)) => num - 1,
                        _ => {
                            println!(
                                "Invalid 'to' stack. Enter another number between 0 and {}.",
                                self.stacks.len() - 1
                            );
                            continue;
                        }
                    };

                    if self.move_requires_more_room(from, to) {
                        println!(
                            "No room in stack {} for stack {}'s top.",
                            to + 1,
                            from + 1,
                        );
                        continue;
                    }

                    if self.move_tops_mismatch(from, to) {
                        println!(
                            "Stack {} top mismatches stack {}.",
                            to + 1,
                            from + 1,
                        );
                        continue;
                    }

                    UserInput {
                        menu_option: MenuOption::Move,
                        stack_move: Some((from, to)),
                    }
                }
            };
            break;
        }
        return user_input;
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
