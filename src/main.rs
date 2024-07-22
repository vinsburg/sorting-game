use sorting_game::Game;

fn main() {
    let mut game = Game::game_0();
    game.turn_loop();
    game = Game::game_1();
    game.turn_loop();
}
