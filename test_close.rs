use iced::event;
use iced::executor;
use iced::widget::{button, column, container, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Subscription, Theme};

pub fn main() -> iced::Result {
    App::run(Settings {
        window: window::Settings {
            exit_on_close_request: false,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug, Default)]
struct App;

#[derive(Debug, Clone)]
enum Message {
    Event(iced::Event),
    Exit,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self, Command::none())
    }

    fn title(&self) -> String {
        "Close Test".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Event(event) => {
                match &event {
                    iced::Event::Window(_, window::Event::CloseRequested) => {
                        println!("[test] CloseRequested received!");
                    }
                    _ => {}
                }
                Command::none()
            }
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("Click X to test close event"),
            button(text("Exit")).on_press(Message::Exit),
        ]
        .spacing(20)
        .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
