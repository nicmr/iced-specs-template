use specs::prelude::*;
use specs::shrev::{EventChannel};

use crossbeam_channel::{Receiver, TryRecvError, Sender};
use super::resources::{TurnResource};

pub enum PlayerAction {
    StartGame,
    IncrementTurn,
}

pub enum Broadcast {
    ShareChanges,
}

#[derive(Debug, Clone)]
pub struct FrontendReceiver {
    pub receiver: Receiver<PlayerAction>,
}

#[derive(Debug, Clone)]
pub struct FrontendSender {
    pub sender: Sender<usize>,
}

pub struct ActionChannelSystem;

impl<'a> System<'a> for ActionChannelSystem {
    type SystemData = (
        Option<Read<'a, FrontendReceiver>>,
        Write<'a, EventChannel<PlayerAction>>
    );

    fn run(&mut self, (frontend_channel, mut event_channel): Self::SystemData) {
        if let Some(chan) = frontend_channel {
            match chan.receiver.try_recv() {
                Ok(action) => event_channel.single_write(action),
                Err(e) => match e {
                    TryRecvError::Empty => (), // there's simply no new message, that's alright
                    TryRecvError::Disconnected =>{
                        println!("(Warning): channel disconnected!"); // this happens on shutdown;
                    }
                }
            }
        } else {
            println!("Frontend channel doesn't exist yet, skipping ActionChannelSystem execution");
        }
    }
}

pub struct SendToFrontendSystem{
    reader_id: ReaderId<Broadcast>,
}

impl SendToFrontendSystem {
    pub fn new(world: &mut World) -> Self {
        <Self as System<'_>>::SystemData::setup(world);
        let reader_id = world.fetch_mut::<EventChannel<Broadcast>>().register_reader();
        Self {reader_id}
    }
}

impl<'a> System<'a> for SendToFrontendSystem {
    type SystemData = (
        Option<Read<'a, FrontendSender>>,
        Read<'a, TurnResource>,
        Read<'a, EventChannel<Broadcast>>,
    );

    fn run(&mut self, (frontend_sender, turn, broadcast_channel): Self::SystemData) {
        if let Some(chan) = frontend_sender {
            for _broadcast in broadcast_channel.read(&mut self.reader_id) {
                match chan.sender.send(turn.number) {
                    Ok(()) => (),
                    Err(_e) => {
                        println!("(Warning): failed to send status update to frontend");
                    },
                }
            }
        } else {
            println!("Frontend channel doesn't exist yet, skipping SendToFrontendSystem execution");
        }
    }
}