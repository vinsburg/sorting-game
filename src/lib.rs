/*
Let's make a game where you have P pillars of size S, and K kinds where each type has U units, scattered across the pillars.
You may move units from pillar p0 to pillar p1 if the top stack units are of the same kind k0, and there is room on p1 for all k0 units from p0.
*/
use std::fmt;

#[derive(Clone, PartialEq, Eq, Copy)]
struct Kind {
    id: usize,
}

impl fmt::Debug for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The debug output of kind is the kind's id.
        write!(f, "{}", self.id)
    }
}

struct Pillar {
    size: usize,
    units: Vec<Kind>,
}

#[allow(dead_code)]
impl Pillar {
    fn is_vacant(&self) -> bool {
        self.units.len() == 0
    }

    fn get_vacancy(&self) -> usize {
        self.size - self.units.len()
    }

    fn get_top_occupant_kind(&self) -> Kind {
        if self.units.len() == 0 {
            return Kind { id: 0 };
        } else {
            return self.units.last().unwrap().clone();
        }
    }

    fn pop_top_occupants(&mut self, occupants: &mut Vec<Kind>) {
        let top_occupant = self.get_top_occupant_kind();
        while !self.is_vacant() && self.get_top_occupant_kind() == top_occupant {
            self.units.pop();
            occupants.push(top_occupant.clone());
        }
    }

    fn push_occupants(&mut self, occupants: &mut Vec<Kind>) {
        while occupants.len() != 0 {
            let occupant = occupants.pop().unwrap();
            self.units.push(occupant);
        }
    }
}

pub struct Game {
    pillars: Vec<Pillar>,
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
        for id in 1..kinds_size + 1 {
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
        println!("Pillars:");
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

    pub fn make_a_move(&mut self, from: usize, to: usize) {
        let occupants = &mut Vec::new();
        let from_top_occupant = self.pillars[from].get_top_occupant_kind();
        self.pillars[from].pop_top_occupants(occupants);
        
        if !self.pillars[to].is_vacant() {
            let to_top_occupant = self.pillars[to].get_top_occupant_kind();
            let to_vacancy: usize = self.pillars[to].get_vacancy();
            if (from_top_occupant != to_top_occupant) || (to_vacancy < occupants.len()) {
                self.pillars[from].push_occupants(occupants);
            }
        }
        if occupants.len() != 0 {
            self.pillars[to].push_occupants(occupants);
        }
    }
}
