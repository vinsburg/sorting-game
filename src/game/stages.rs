use crate::game::stack::Stack;
use crate::game::{Game, LineReader};

impl <TLR: LineReader + Default> Game<TLR> {
    fn vecs_to_stacks(vecs: Vec<Vec<usize>>) -> Vec<Stack> {
        let mut stacks: Vec<Stack> = Vec::new();
        let mut stack: Stack;
        for vec in vecs {
            stack = Stack::new_from_vec(vec);
            stacks.push(stack);
        }
        stacks
    }

    fn new_from_vecs(vecs: Vec<Vec<usize>>, stage_name: Option<String>, line_reader: TLR) -> Game<TLR> {
        Game::new(Game::<TLR>::vecs_to_stacks(vecs), stage_name, line_reader)
    }

    pub fn get_stages() -> Vec<Game<TLR>> {
        let stage_vec: Vec<Vec<Vec<usize>>> = vec![
            vec![vec![2, 1, 0], vec![1, 2], vec![2, 0]],
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
        let mut stages: Vec<Game<TLR>> = Vec::new();
        for (ind, vec_stacks) in stage_vec.iter().enumerate() {
            let name: String = format!("Stage - {}", ind + 1);
            stages.push(Game::new_from_vecs(vec_stacks.clone(), Some(name), TLR::default()));
        }
        stages
    }
}
