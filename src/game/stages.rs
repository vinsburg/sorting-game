use crate::game::stack::kind::Kind;
use crate::game::stack::Stack;
use crate::game::Game;

impl Game {
    fn vecs_to_stacks(vecs: Vec<Vec<usize>>) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = Vec::new();
        for vec in vecs {
            let vec_len = vec.len();
            let mut units: Vec<Kind> = Vec::new();
            for unit_id in vec {
                let kind = Kind::new(unit_id);
                if !kind.is_empty() {
                    units.push(kind);
                }
            }
            stacks.push(Stack {
                size: vec_len,
                units,
            });
        }
        stacks
    }

    pub fn stage_0() -> Game {
        let vec_stacks = vec![vec![2, 1, 0], vec![1, 2, 0], vec![2, 0]];
        let stacks = Game::vecs_to_stacks(vec_stacks);
        Game::new(stacks)
    }

    pub fn stage_1() -> Game {
        let vec_stacks = vec![
            vec![1, 2, 3, 0, 0],
            vec![5, 5, 3, 3, 4],
            vec![6, 7, 8, 2, 8],
            vec![9, 7, 7, 0, 0],
            vec![2, 7, 1, 10, 0],
            vec![9, 5, 5, 3, 9],
            vec![7, 3, 10, 9, 0],
            vec![0, 0, 0, 0, 0],
            vec![6, 6, 1, 0, 0],
            vec![5, 8, 6, 0, 0],
            vec![8, 4, 9, 0, 0],
            vec![10, 10, 8, 6, 1],
            vec![2, 4, 1, 10, 0],
            vec![4, 2, 4, 0, 0],
        ];
        let stacks = Game::vecs_to_stacks(vec_stacks);
        Game::new(stacks)
    }

    pub fn stage_9() -> Game {
        let vec_stacks = vec![
            vec![1, 2, 3, 0, 0],
            vec![5, 5, 3, 3, 4],
            vec![6, 7, 8, 2, 8],
            vec![9, 7, 7, 0, 0],
            vec![2, 7, 1, 10, 0],
            vec![9, 5, 5, 3, 9],
            vec![7, 3, 10, 9, 0],
            vec![6, 6, 1, 0, 0],
            vec![5, 8, 6],
            vec![8, 4, 9],
            vec![10, 10, 8, 6, 1],
            vec![2, 4, 1, 10, 0],
            vec![4, 2, 4],
        ];
        let stacks = Game::vecs_to_stacks(vec_stacks);
        Game::new(stacks)
    }
}
