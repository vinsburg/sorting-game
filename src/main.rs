use sorting_game::game::Game;

fn main() {
    // TODO: move the game loop to the Game mod.rs file
    let stages = Game::get_stages();
    let last_stage_index = stages.len() - 1;
    for (ind, mut stage) in stages.into_iter().enumerate() {
        stage.turn_loop();
        stage.stage_complete_prompt(ind == last_stage_index);
    }
}
