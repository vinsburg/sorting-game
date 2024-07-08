use sorting_game::Game;

fn main() {
    // TODO: init the game with random state based on seed / with set of stacks.
    let stack_quantity = 4;
    let stack_size = 4;
    let units_per_kind = 4;
    let kinds_size = 3;
    let seed = 0;

    let mut game = Game::new(stack_quantity, stack_size, units_per_kind, kinds_size, seed);
    game.turn_loop();
}
