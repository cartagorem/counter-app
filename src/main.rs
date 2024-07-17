use iced::widget::{button, column, text, text_input, Column};
use iced::{Element, Sandbox, Settings};

#[derive(Debug, Default)]
struct Counter {
    value: i64,
    input_text: String,
    element_text: String,
    generated_text: String,
}

use std::fmt;
impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Counter(value={})", self.value)
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    Reset,
    TextInputChanged(String),
    Enter,
    Generate,
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
            Message::Reset => self.reset_btn(),
            Message::TextInputChanged(input_entered_text) => self.input_text = input_entered_text,
            Message::Enter => self.enter_btn(),
            Message::Generate => self.generate_text_btn(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        self.column().padding(32).into()
    }
}

impl Counter {
    fn column(&self) -> Column<Message> {
        column![
            // Counter
            button("+").on_press(Message::Increment),
            text(self.value).size(20),
            button("-").on_press(Message::Decrement),
            button("Reset").on_press(Message::Reset),
            // Input
            text_input("Enter an input", &self.input_text).on_input(Message::TextInputChanged),
            button("Enter").on_press(Message::Enter),
            text(self.element_text.clone()).size(20),
            // Generate input
            button("Generate").on_press(Message::Generate),
            text(&self.generated_text).size(20),
        ]
    }
}

// helpers
impl Counter {
    fn enter_btn(&mut self) {
        self.element_text = self.input_text.clone();
        self.input_text.clear();
    }

    fn reset_btn(&mut self) {
        self.value = 0;
        self.input_text.clear();
        self.element_text.clear();
    }

    fn generate_text_btn(&mut self) {
        self.generated_text = format!("Counter {{ value: {} }}", self.value);
    }
}

fn main() -> iced::Result {
    Counter::run(Settings::default())
}
