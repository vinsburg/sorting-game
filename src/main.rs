use sorting_game::game::Game;

fn main() {
    let stages = Game::get_stages();
    for mut stage in stages {
        stage.turn_loop();
    }
}
