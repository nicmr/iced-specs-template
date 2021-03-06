use specs::prelude::*;
use specs_derive::Component;
use specs::shrev::{EventChannel};

use crossbeam_channel::{Sender, Receiver};

/// Modules related to io interfacing with the ECS
pub mod io;
/// Components and Systems related to game logic
pub mod game;

pub mod resources;

use io::{PlayerAction, Broadcast};



#[derive(Component, Debug, Clone)]
pub struct PlaceholderComponent {
    pub name: &'static str,
}

pub struct ColonyState {
    pub world: World,
}

pub fn default_state_and_dispatcher<'a> (input_receiver: Receiver<PlayerAction>, event_sender: Sender<usize>) -> (ColonyState, Dispatcher<'a, 'a>) {
    let mut gs = ColonyState {
        world: World::new(),
    };
    gs.world.register::<PlaceholderComponent>();
    gs.world.insert(io::FrontendReceiver{receiver: input_receiver});
    gs.world.insert(io::FrontendSender{sender: event_sender});
    gs.world.insert(EventChannel::<PlayerAction>::new());
    gs.world.insert(EventChannel::<Broadcast>::new());

    gs.world.create_entity()
        .with(PlaceholderComponent { name: "Barkeeper"})
        .build();

    gs.world.create_entity()
        .with(PlaceholderComponent { name: "Guard"})
        .build();

    let mut dispatcher =
        DispatcherBuilder::new()
        .with(io::ActionChannelSystem, "ActionChannelSystem", &[])
        .with(
            game::TurnIncrementSystem::new(&mut gs.world),
            "TurnIncrementSystem",
            &["ActionChannelSystem"]
        ).with(
            io::SendToFrontendSystem::new(&mut gs.world),
            "SendToFrontendSystem",
            &["TurnIncrementSystem"]
        )
        .build();

    dispatcher.setup(&mut gs.world);
    (gs, dispatcher)
}