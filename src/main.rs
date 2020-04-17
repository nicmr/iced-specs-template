use std::thread;

use crossbeam_channel::{unbounded, Sender, Receiver};
use iced::{
    button, Application, Button, Column, Command,
    Container, Element, Length,Settings, Text, window,
};

mod colony;
use colony::io::PlayerAction;

fn main() {
    let (input_sender, input_receiver) = unbounded();
    let (event_sender, event_receiver) = unbounded();

    let _handle = thread::spawn(move || {

        let (game_state, mut dispatcher) = colony::default_state_and_dispatcher(input_receiver, event_sender);

        loop {
          dispatcher.dispatch(&game_state.world);
        }
    });


    ColonyFrontend::run(
        Settings{
            window: window::Settings::default(),
            flags: (input_sender, event_receiver),
            default_font: None,
            antialiasing: false,
        }
    );
}



struct ColonyFrontend {
    input_sender: Sender<PlayerAction>,
    event_receiver: Receiver<usize>,
    state : FrontendState,
}

enum FrontendState {
    NothingHere{btn_state: button::State},
    NormalTurn{btn_state: button::State, turn: usize},
}

#[derive(Debug, Clone)]
enum Message {
    StartGame,
    GameStarted(Result<(), Error>),
    MessageSent(Result<(), Error>),
    NextTurn,
    TurnActive(Result<usize, Error>),
}

impl Application for ColonyFrontend {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = (Sender<PlayerAction>, Receiver<usize>);

    fn new ((input_sender, event_receiver): Self::Flags) -> (ColonyFrontend, Command<Message>) {
        (ColonyFrontend {
            input_sender,
            event_receiver,
            state: FrontendState::NothingHere {
                btn_state: button::State::new(),
            },
        }, Command::none()
        )
    }

    fn title(&self) -> String {
        let subtitle = match self.state {
            FrontendState::NothingHere{btn_state: _} => "NothingHere",
            FrontendState::NormalTurn{btn_state: _, turn: _} => "Playing turn ..",
        };
        format!("{} - Colony", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::StartGame => {
                println!("start game pressed");
                Command::perform(ColonyFrontend::start_game(self.input_sender.clone()), Message::GameStarted)
            },
            Message::GameStarted(_) => Command::perform(ColonyFrontend::get_turn(self.event_receiver.clone()), Message::TurnActive),
            Message::NextTurn => Command::perform(ColonyFrontend::next_turn(self.input_sender.clone()), Message::MessageSent),
            Message::MessageSent(success) => {
                match success {
                    Ok(()) => (),
                    Err(err) => {
                        println!("{:?}", err);
                    }
                }
                Command::none()
            },
            Message::TurnActive(Ok(turn)) => {
                self.state = match self.state {
                    FrontendState::NothingHere{btn_state} | FrontendState::NormalTurn{btn_state, turn:  _} => FrontendState::NormalTurn{btn_state, turn}
                };
                Command::perform(ColonyFrontend::get_turn(self.event_receiver.clone()), Message::TurnActive)
            },
            Message::TurnActive(Err(e)) => {
                println!("{:?}", e);
                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match &mut (self.state) {
            FrontendState::NothingHere{btn_state} =>
                Column::new()
                    .width(Length::Shrink)
                    .push(Text::new("Nothing here..."))
                    .push( button(btn_state, "Start game").on_press(Message::StartGame) )
            ,
            FrontendState::NormalTurn{ref mut btn_state, turn} =>
                Column::new()
                    .width(Length::Shrink)
                    .push(Text::new(format!("Turn: {}", turn)))
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
    async fn start_game(action_sender: Sender<PlayerAction>) -> Result<(), Error> {
        match action_sender.send(PlayerAction::StartGame)  {
            Ok(()) => Ok(()),
            Err(_) => Err(Error{desc: "Failed to send StartGame message"}),
        }
    }
    async fn get_turn(event_receiver: Receiver<usize>) -> Result<usize, Error> {
        match event_receiver.recv() {
            Ok(val) => Ok(val),
            Err(_e) => {
                Err(Error{desc: "Frontend failed to read from channel"})
            }            
        }
    }
    async fn next_turn(action_sender: Sender<PlayerAction>) -> Result<(), Error> {
        match action_sender.send(PlayerAction::IncrementTurn)  {
            Ok(()) => Ok(()),
            Err(_) => Err(Error{desc: "Failed to send IncrementTurn message"}),
        }      
    }
}

#[derive(Debug, Clone)]
struct Error {
    desc: &'static str
}