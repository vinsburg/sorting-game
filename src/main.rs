use sorting_game::Game;

fn main() {
    let pillar_quantity = 3;
    let pillar_size = 3;
    let units_per_kind = 3;
    let kinds_size = 2;
    let seed = 0;

    let mut game = Game::new(
        pillar_quantity,
        pillar_size,
        units_per_kind,
        kinds_size,
        seed,
    );
    game.render();
    game.move_units(0, 2);
    game.render()
}
