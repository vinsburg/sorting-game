use crate::game::Game;
use std::io::{self, Write};
use crate::line_reader::LineReader;

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

impl<TLR: LineReader + Default> Game<TLR> {
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
            for unit_id in stack.iter_unit_ids() {
                let unit_index: usize = self.get_kind_index(unit_id);
                let color: [u8; 3] = COLORS[unit_index % COLORS.len()].clone();
                buffer.push_str(
                    format!(
                        "\x1b[38;2;{};{};{}m{:>2}\x1b[0m ",
                        color[0], color[1], color[2], unit_id,
                    )
                    .as_str(),
                );
            }
            for _ in 0..stack.get_vacancy() {
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

        self.render();
        println!(
            "All Stacks Sorted! - {}\nPress Enter to continue",
            game_complete_message
        );
        self.line_reader.read_line(&mut String::new());
    }

    pub fn show_help(&self) {
        self.render();
        println!("Help:\n");
        println!(
            "Enter two numbers separated by a space to move units between stacks (e.g., '2 3')"
        );
        println!("Type 'u' to undo the last move");
        println!("Type 'r' to reset the stage");
        println!("Type 'q' to quit the game");
        println!("Press Enter to continue");
        self.line_reader.read_line(&mut String::new());
    }

    pub fn read_valid_input(&self) -> UserInput {
        let mut user_input: UserInput = UserInput::new_menu_option(MenuOption::Help);
        let mut input: String = String::new();
        let default_prompt: String = "Input move or type 'h' for help".to_string();
        let invalid_input_prompt: String = format!(
            "Invalid input!\nPlease enter two different numbers between 1 and {} separated by a space", self.stacks.len()
        );
        let mut current_prompt: String;
        let mut next_prompt: String = String::new();

        loop {
            if self.stage_complete() {
                break;
            }

            let no_legal_moves: bool = self.no_legal_moves();
            if no_legal_moves {
                next_prompt =
                    "No legal moves left! - press 'u' to undo or 'r' to restart.".to_string();
            }

            match next_prompt.len() {
                0 => current_prompt = default_prompt.clone(),
                _ => {
                    current_prompt = next_prompt.clone();
                    next_prompt.clear();
                }
            }

            self.render();
            print!("{}: ", current_prompt);

            let str_input: &str = match no_legal_moves {
                // TODO: show help when no legal moves, handle flushing outside.
                _ => {
                    io::stdout().flush().unwrap(); // Flush to ensure the message is displayed before reading input
                    input.clear();
                    self.line_reader.read_line(&mut input);
                    input.trim()
                }
            };

            user_input = match str_input {
                "h" => UserInput::new_menu_option(MenuOption::Help),
                "q" => UserInput::new_menu_option(MenuOption::Quit),
                "r" => UserInput::new_menu_option(MenuOption::Reset),
                "u" => UserInput::new_menu_option(MenuOption::Undo),
                _ => {
                    let parts: Vec<&str> = input.trim().split_whitespace().collect();
                    if parts.len() != 2 {
                        next_prompt = invalid_input_prompt.clone();
                        continue;
                    }

                    let from = match parts[0].parse::<usize>() {
                        Ok(num) if num - 1 < self.stacks.len() => num - 1,
                        _ => {
                            next_prompt = invalid_input_prompt.clone();
                            continue;
                        }
                    };

                    let to = match parts[1].parse::<usize>() {
                        Ok(num) if ((num - 1 < self.stacks.len()) && (num - 1 != from)) => num - 1,
                        _ => {
                            next_prompt = invalid_input_prompt.clone();
                            continue;
                        }
                    };

                    if self.move_requires_more_room(from, to) {
                        next_prompt =
                            Game::<TLR>::illegal_move_prompt("Not enough room in the target stack");
                        continue;
                    }

                    if self.stack_tops_mismatch(from, to) {
                        next_prompt = Game::<TLR>::illegal_move_prompt(
                            "Units can only be moved towards identical units, or empty stacks",
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

    fn illegal_move_prompt(prompt: &str) -> String {
        format!("Illegal move!\n{}.\nplease try again", prompt)
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
