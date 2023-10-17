use iced::widget::{Column, Text};
use iced::{Element, Sandbox, Settings};

fn main() -> iced::Result {
    Model::run(Settings::default())
}

struct Model {
    count: i64,
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Incr,
    Decr,
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {
        Model { count: 1 }
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Msg::Incr => self.count += 1,
            Msg::Decr => self.count -= 1,
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let label = Text::new(self.count.to_string());
        Column::new().push(label).into()
    }
}
