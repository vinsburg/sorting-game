use sorting_game::Game;

fn main() {
    let stack_quantity = 3;
    let stack_size = 3;
    let units_per_kind = 3;
    let kinds_size = 2;
    let seed = 0;

    let mut game = Game::new(stack_quantity, stack_size, units_per_kind, kinds_size, seed);
    game.turn_loop();
}
