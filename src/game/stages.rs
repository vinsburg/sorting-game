use crate::game::stack::kind::Kind;
use crate::game::stack::Stack;
use crate::game::Game;

impl Game {
    fn vecs_to_stacks(vecs: Vec<Vec<usize>>) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = Vec::new();
        for vec in vecs {
            let vec_len: usize = vec.len();
            let mut units: Vec<Kind> = Vec::new();
            for unit_id in vec {
                let kind: Kind = Kind::new(unit_id, 1);
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

    fn new_from_vecs(vecs: Vec<Vec<usize>>, stage_name: Option<String>) -> Game {
        Game::new(Game::vecs_to_stacks(vecs), stage_name)
    }

    pub fn get_stages() -> Vec<Game> {
        let stage_vec: Vec<Vec<Vec<usize>>> = vec![
            vec![vec![2, 1, 0], vec![1, 2, 0], vec![2, 0]],
            vec![
                vec![1, 2, 3, 0, 0],
                vec![4, 4, 3, 3, 5],
                vec![6, 7, 8, 2, 8],
                vec![9, 7, 7, 0, 0],
                vec![2, 7, 1, 10, 0],
                vec![9, 4, 4, 3, 9],
                vec![7, 3, 10, 9, 0],
                vec![0, 0, 0, 0, 0],
                vec![6, 6, 1, 0, 0],
                vec![4, 8, 6, 0, 0],
                vec![8, 5, 9, 0, 0],
                vec![10, 10, 8, 6, 1],
                vec![2, 5, 1, 10, 0],
                vec![5, 2, 5, 0, 0],
            ],
            vec![
                vec![1, 2, 3, 0, 0],
                vec![4, 4, 3, 3, 5],
                vec![6, 7, 8, 2, 8],
                vec![9, 7, 7, 0, 0],
                vec![2, 7, 1, 10, 0],
                vec![9, 4, 4, 3, 9],
                vec![7, 3, 10, 9, 0],
                vec![6, 6, 1, 0, 0],
                vec![4, 8, 6],
                vec![8, 5, 9],
                vec![10, 10, 8, 6, 1],
                vec![2, 5, 1, 10, 0],
                vec![5, 2, 5],
            ],
        ];
        let mut stages: Vec<Game> = Vec::new();
        for (ind, vec_stacks) in stage_vec.iter().enumerate() {
            let name: String = format!("Stage - {}", ind+1);
            stages.push(Game::new_from_vecs(vec_stacks.clone(), Some(name)));
        }
        stages
    }
}
