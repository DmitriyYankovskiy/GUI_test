use iced::widget::{button, column, text, Container, canvas::{Frame}};
use iced::{executor, Alignment, Application, Command, Element, Settings, Theme, };

fn main() -> iced::Result {
    Model::run(Settings::default())
}

struct Model {
    counter: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Application for Model {
    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Self::Message::Increment => self.counter += 1,
            Self::Message::Decrement => self.counter -= 1,
        };
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Container::new(
            column![
                button("+").on_press(Self::Message::Increment),
                text(self.counter),
                button("-").on_press(Self::Message::Decrement),
            ]
            .align_items(Alignment::Center),
        )
        .center_x()
        .center_y()
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .into()
    }

    fn new(_: ()) -> (Model, Command<Self::Message>) {
        (Self { counter: 0 }, Command::none())
    }

    type Message = Message;

    type Executor = executor::Default;

    type Flags = ();

    type Theme = Theme;
}
