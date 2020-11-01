use iced::{button, Align, Button, Column, Element, Length, Row, Settings, Text};
use iced::{executor, Application, Command};
#[derive(Default)]
pub struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Self::Message>) {
        let state = Counter {
            value: 0,
            increment_button: button::State::new(),
            decrement_button: button::State::new(),
        };

        (state, Command::none())
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        };

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let inc_button = Button::new(&mut self.increment_button, Text::new("+"))
            .on_press(Message::IncrementPressed);

        let dec_button = Button::new(&mut self.decrement_button, Text::new("-"))
            .on_press(Message::DecrementPressed);

        let counter_text = Text::new(self.value.to_string())
            .size(50)
            .width(Length::Units(100));

        let info_row = Row::new().align_items(Align::Center).push(counter_text);
        let action_row = Row::new().push(inc_button).push(dec_button);

        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(info_row)
            .push(action_row)
            .into()
    }
}

pub fn run_counter() {
    Counter::run(Settings::default())
}
