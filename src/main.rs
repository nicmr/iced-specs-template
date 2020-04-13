use std::thread;

use specs::prelude::*;
use crossbeam_channel::{unbounded, Sender, Receiver};
use iced::{
    button, futures, image, Align, Application, Button, Column, Command,
    Container, Element, Image, Length, Row, Settings, Text, window,
};

mod colony;
use colony::{ ColonyState, Occupation, OccupationPrintSystem};

struct SubscribeMessage;

fn main() {
    let (sub_sender, _sub_receiver) = unbounded();
    let (event_sender, event_receiver) = unbounded();

    thread::spawn(move || {
        let mut gs = ColonyState {
            world: World::new(),
        };
        gs.world.register::<Occupation>();
    
        gs.world.create_entity()
            .with(Occupation { name: "Barkeeper"})
            .build();
    
        gs.world.create_entity()
            .with(Occupation { name: "Guard"})
            .build();
    
        let mut dispatcher =
            DispatcherBuilder::new()
                .with(OccupationPrintSystem, "occprs", &[])
                .build();
    
        dispatcher.setup(&mut gs.world);

        event_sender.send(0);

        // loop {
        //     let register = receiver.recv();
        //     println!("Somebody tried to register");
        // }
    });


    ColonyFrontend::run(
        Settings{
            window: window::Settings::default(),
            flags: (event_receiver, sub_sender),
            default_font: None,
            antialiasing: false,
        }
    )
}

struct ColonyFrontend {
    subscription_sender: Sender<()>,
    event_receiver: Receiver<usize>,
    state : FrontendState,
}

enum FrontendState {
    NothingHere{btn_state: button::State},
    NormalTurn{btn_state: button::State},
}

#[derive(Debug, Clone)]
enum Message {
    TurnActive(Result<usize, Error>),
    NextTurn,
}

impl Application for ColonyFrontend {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (Receiver<usize>, Sender<()>);

    fn new ((event_receiver, subscription_sender): Self::Flags) -> (ColonyFrontend, Command<Message>) {
        (ColonyFrontend {
            event_receiver,
            subscription_sender,
            state: FrontendState::NothingHere {
                btn_state: button::State::new(),
            },
        }, Command::none()
        )
    }

    fn title(&self) -> String {
        let subtitle = match self.state {
            FrontendState::NothingHere{btn_state: _} => "NothingHere",
            FrontendState::NormalTurn{btn_state: _} => "Playing turn ..",
        };
        format!("{} - Colony", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TurnActive(Ok(_turn)) => {
                self.state = match self.state {
                    FrontendState::NothingHere{btn_state} | FrontendState::NormalTurn{btn_state} => FrontendState::NormalTurn{btn_state}
                };
                Command::none()
            },
            Message::TurnActive(Err(_e)) => {
                println!("I failed ... ");
                Command::none()
            }
            Message::NextTurn => Command::perform(ColonyFrontend::get_turn(self.event_receiver.clone()), Message::TurnActive)
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &mut (self.state) {
            FrontendState::NothingHere{btn_state} =>
                Column::new()
                    .width(Length::Shrink)
                    .push(Text::new("Nothing here..."))
                    .push( button(btn_state, "Start game").on_press(Message::NextTurn) )
            ,
            FrontendState::NormalTurn{ref mut btn_state} =>
                Column::new()
                    .width(Length::Shrink)
                    .push(Text::new("Turn:"))
                    .push( button(btn_state, "Next turn").on_press(Message::NextTurn) ),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}


fn button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text))
        .padding(10)
}

impl ColonyFrontend {
    async fn get_turn(event_receiver: Receiver<usize>) -> Result<usize, Error> {

        match event_receiver.recv() {
            Ok(val) => Ok(val),
            Err(_e) => {
                println!("Frontend failed to read from channel");
                Err(Error{})
            }            
        }
    }
}

#[derive(Debug, Clone)]
struct Error;