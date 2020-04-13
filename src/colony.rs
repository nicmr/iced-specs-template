use specs::prelude::*;
use specs_derive::Component;
use specs::shrev::{EventChannel};

use crossbeam_channel::{Sender, Receiver};

/// Modules related to io interfacing with the ECS
pub mod io;
/// Components and Systems related to game logic
pub mod game;

pub mod resources;

use io::{PlayerAction};



#[derive(Component, Debug, Clone)]
pub struct Occupation {
    pub name: &'static str,
}

pub struct ColonyState {
    pub world: World,
}

pub fn default_state_and_dispatcher<'a> (input_receiver: Receiver<PlayerAction>, event_sender: Sender<usize>) -> (ColonyState, Dispatcher<'a, 'a>) {
    let mut gs = ColonyState {
        world: World::new(),
    };
    gs.world.register::<Occupation>();
    // gs.world.insert(input_receiver);
    gs.world.insert(io::FrontendReceiver{receiver: input_receiver});
    gs.world.insert(EventChannel::<PlayerAction>::new());

    gs.world.create_entity()
        .with(Occupation { name: "Barkeeper"})
        .build();

    gs.world.create_entity()
        .with(Occupation { name: "Guard"})
        .build();

    let mut dispatcher =
        DispatcherBuilder::new()
        // .with(io::OccupationPrintSystem, "occprs", &[])
        .with(io::ActionChannelSystem, "ActionChannelSystem", &[])
        .with(
            game::TurnIncrementSystem::new(&mut gs.world),
            "TurnIncrementSystem",
            &["ActionChannelSystem"]
        ).build();

    dispatcher.setup(&mut gs.world);
    (gs, dispatcher)
}