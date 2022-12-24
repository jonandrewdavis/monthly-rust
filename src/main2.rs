use iced::widget::canvas::{Cache, Cursor, Geometry, Path};
use iced::widget::{
    self, button, canvas, checkbox, column, container, row, scrollable, text, text_input, Button,
    Text,
};
use iced::{alignment, executor, theme, window, Color};
use iced::{Application, Command, Element, Length, Rectangle, Settings, Theme};
struct Monthly {
    geo: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {}

fn main() -> iced::Result {
    let measure = Measure {
        width: 520,
        height: 180,
    };

    Monthly::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (measure.width, measure.height),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Measure {
    height: u32,
    width: u32,
}

impl Application for Monthly {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        return (
            Monthly {
                geo: Default::default(),
            },
            Command::none(),
        );
    }

    fn title(&self) -> String {
        String::from("Monthly")
    }

    fn view(&self) -> Element<Self::Message> {
        let title = text("monthly")
            .width(Length::Fill)
            .size(32)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Left);

        // let canvas = canvas(self)
        //     .width(Length::Units(200))
        //     .height(Length::Units(800));

        // add a button to import a file

        let import_button = {
            let label: Text = text("Import").size(16);
            let button = button(label);
            button.on_press(()).padding(8)
        };

        let content = column![
            title,
            text("Loading...")
                .horizontal_alignment(alignment::Horizontal::Center)
                .size(50),
            import_button
        ]
        .spacing(20)
        .max_width(800);

        container(content)
            .width(Length::Units(200))
            .height(Length::Units(800))
            .padding(20)
            .into()
    }

    // fn view(&self) -> Element<Message> {
    //     // let canvas = canvas(self as &Self)
    //     //     .width(Length::Fill)
    //     //     .height(Length::Fill);

    //     let filter_button = {
    //         let label: Text = text("Import").size(16);
    //         let button = button(label);
    //         button
    //     };

    //     container(filter_button)
    //         .width(Length::Fill)
    //         .height(Length::Fill)
    //         .padding(20)
    //         .into()
    // }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

impl<Message> canvas::Program<Message> for Monthly {
    type State = ();

    fn draw(
        &self,
        state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let base = self.geo.draw(bounds.size(), |frame| {});
        vec![base]
    }
}
