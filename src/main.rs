use std::process::Command;
use iced::widget::{row, column, text, Slider};
use iced::{Alignment, Element, Sandbox, Settings};

fn change_led(r:u8, g:u8, b:u8){
    Command::new("ds4led")
    .args([r.to_string(), g.to_string(), b.to_string()])
    .output()
    .expect("failed to execute process");
}

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChangedR(u8),
    SliderChangedG(u8),
    SliderChangedB(u8),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {r: 0, g: 0, b: 0 }
    }

    fn title(&self) -> String {
        String::from("ds4led - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChangedR(c) => {
                self.r = c;
            }
            Message::SliderChangedG(c) => {
                self.g = c;
            }
            Message::SliderChangedB(c) => {
                self.b = c;
            }
        }
        change_led(self.r, self.g, self.b)
    }

    fn view(&self) -> Element<Message> {
        
        column![
            row![
                text("R:").size(50),
                text(self.r).size(50),
                text("G:").size(50),
                text(self.g).size(50),
                text("B:").size(50),
                text(self.b).size(50),
            ].align_items(Alignment::Fill).spacing(20),
            Slider::new(0..=255, self.r, Message::SliderChangedR),
            Slider::new(0..=255, self.g, Message::SliderChangedG),
            Slider::new(0..=255, self.b, Message::SliderChangedB),
        ]
        .padding(20)
        .spacing(40)
        .align_items(Alignment::Center)
        .into()
    }
}