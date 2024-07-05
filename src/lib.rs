/*
Let's make a game where you have P pillars of size S, and K kinds where each type has U units, scattered across the pillars.
You may move units from pillar p0 to pillar p1 if the top stack units are of the same kind k0, and there is room on p1 for all k0 units from p0.
*/
use std::fmt;

#[derive(Clone)]
struct Kind {
    id: usize,
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Write the id directly, making Kind's debug output just its id as a usize
        write!(f, "{}", self.id)
    }
}

struct Pillar {
    size: usize,
    units: Vec<Kind>,
}

pub struct Game {
    pillars: Vec<Pillar>,
    // kinds: Vec<Kind>,
}

impl Game {
    pub fn new(
        pillar_quantity: usize,
        pillar_size: usize,
        units_per_kind: usize,
        kinds_size: usize,
        seed: u32,
    ) -> Game {
        println!("Seed: {}", seed); // TODO: use seed to randomize the game.
                                    // let mut kinds = Vec::new();
        let mut units = Vec::new();
        for id in 0..kinds_size {
            // kinds.push(Kind { id: id });
            for _ in 0..units_per_kind {
                units.push(Kind { id: id });
            }
        }

        let units_per_pillar: usize = (kinds_size * units_per_kind) / pillar_quantity;
        let mut pillars = Vec::new();
        for _ in 0..pillar_quantity {
            let mut pillar_units = Vec::new();
            for _ in 0..units_per_pillar {
                let unit = units.pop().unwrap();
                pillar_units.push(unit);
            }
            pillars.push(Pillar {
                size: pillar_size,
                // Init a vec of type Unit:
                units: pillar_units,
            });
        }

        Game { pillars } //, kinds }
    }

    pub fn render(&self) {
        for (pillar_ind, pillar) in self.pillars.iter().enumerate() {
            let mut render_vec: Vec<String> = Vec::new();
            for unit in &pillar.units {
                render_vec.push(format!("{:?}", unit));
            }
            for _ in pillar.units.len()..pillar.size {
                render_vec.push("_".to_string());
            }
            println!("{}: {:?}", pillar_ind, render_vec);
        }
    }
}
