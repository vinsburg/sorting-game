use sorting_game::game::Game;

fn main() {
    let games = vec![
        Game::game_0(),
        Game::game_1(),
        Game::game_9(),
    ];
    for mut game in games {
        game.turn_loop();
    }
}
