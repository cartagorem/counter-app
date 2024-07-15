use iced::widget::{button, column, text};
use iced::{Element, Sandbox, Settings};

#[derive(Debug, Default)]
struct Counter {
    value: i64,
}

use std::fmt;
impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Counter(value={})", self.value)
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Counter::default()
    }

    fn title(&self) -> String {
        String::from("Counter App")
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value).size(20),
            button("-").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
        ]
        .padding(32)
        .into()
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
