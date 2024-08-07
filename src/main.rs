use sorting_game::game::Game;

fn main() {
    let games = vec![Game::stage_0(), Game::stage_1(), Game::stage_9()];
    for mut game in games {
        game.turn_loop();
    }
}
