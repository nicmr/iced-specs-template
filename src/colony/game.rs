
use specs::prelude::*;
use specs::shrev::{EventChannel, ReaderId};

use super::io::{PlayerAction, Broadcast};
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
        Write<'a, EventChannel<Broadcast>>,
    );

    fn run(&mut self, (mut turn, player_actions, mut broadcast): Self::SystemData) {
        for action in player_actions.read(&mut self.reader_id) {
            match action {
                PlayerAction::IncrementTurn => {
                    println!("turn: {}", turn.number);
                    turn.number += 1;
                    broadcast.single_write(Broadcast::ShareChanges);
                },
                PlayerAction::StartGame => {
                    println!("Starting game.");
                    println!("turn: {}", turn.number);
                    broadcast.single_write(Broadcast::ShareChanges);
                }
            }
        }
    }
}