
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use super::io::PlayerAction;
use super::resources::TurnResource;

pub struct TurnIncrementSystem{
    reader_id: ReaderId<PlayerAction>
}

impl TurnIncrementSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<PlayerAction>>().register_reader();
        Self {reader_id}
    }
}

impl<'a> System<'a> for TurnIncrementSystem {
    type SystemData = (
        Write<'a, TurnResource>,
        Read<'a, EventChannel<PlayerAction>>,
    );

    fn run(&mut self, (mut turn, event_channel): Self::SystemData) {
        for event in event_channel.read(&mut self.reader_id) {
            match event {
                PlayerAction::IncrementTurn => {
                    println!("turn: {}", turn.number);
                    turn.number += 1;
                }
            }
        }
    }
}