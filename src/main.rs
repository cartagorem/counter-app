use iced::widget::{button, column, text, Column};

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

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
            Message::Reset => self.value = 0,
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            button("+").on_press(Message::Increment),
            text(self.value),
            button("-").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
        ]
        .padding(32)
    }
}

fn main() -> Result<(), iced::Error> {
    iced::run("Counter App", Counter::update, Counter::view)
}
