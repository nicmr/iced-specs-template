use specs::prelude::*;
use specs::shrev::{EventChannel};

use crossbeam_channel::{Receiver, TryRecvError, Sender};
use super::Occupation;

pub struct OccupationPrintSystem;

impl<'a> System<'a> for OccupationPrintSystem {
    type SystemData = ReadStorage<'a, Occupation>;

    fn run(&mut self, occupations : Self::SystemData) {
        for occ in occupations.join() {
            println!("Occupation is: {}", occ.name);
        }
    }
}

pub struct ActionChannelSystem;

pub enum PlayerAction {
    IncrementTurn,
}

#[derive(Debug, Clone)]
pub struct FrontendReceiver {
    pub receiver: Receiver<PlayerAction>,
}

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
                        println!("Channel disconnected!"); // this shouldn't happen, except possibly on shutdown (?)
                    }
                }
            }
        } else {
            println!("Frontend channel doesn't exist yet, skipping ActionChannelSystem execution");
        }

    }
}

#[derive(Debug, Clone)]
pub struct FrontendSender {
    pub sender: Sender<usize>,
}