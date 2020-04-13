use specs::prelude::*;
use specs_derive::Component;



#[derive(Component, Debug, Clone)]
pub struct Occupation {
    pub name: &'static str,
}

pub struct ColonyState {
    pub world: World,
}

pub struct OccupationPrintSystem;

impl<'a> System<'a> for OccupationPrintSystem {
    type SystemData = ReadStorage<'a, Occupation>;

    fn run(&mut self, occupations : Self::SystemData) {
        for occ in occupations.join() {
            println!("Occupation is: {}", occ.name);
        }

    }
}