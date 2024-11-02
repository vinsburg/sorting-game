use sorting_game::game::Game;
use sorting_game::line_reader::STDInReader;
fn main() {
    Game::<STDInReader>::play(STDInReader::default());
}
