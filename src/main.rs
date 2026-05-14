mod app;
mod i18n;
mod network;
mod pages;
mod theme;
mod types;
mod widgets;

pub fn main() -> iced::Result {
    use iced::Application;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("conduit=info".parse().unwrap())
                .add_directive("wgpu=error".parse().unwrap())
                .add_directive("naga=error".parse().unwrap()),
        )
        .init();

    app::ForwarderApp::run(iced::Settings {
        fonts: vec![
            include_bytes!("../assets/fonts/LXGWWenKaiLite-Regular.ttf")
                .as_slice()
                .into(),
            include_bytes!("../assets/fonts/NotoSansSymbols2-Regular.ttf")
                .as_slice()
                .into(),
        ],
        default_font: iced::Font::with_name("LXGW WenKai Lite"),
        window: iced::window::Settings {
            min_size: Some(iced::Size::new(800.0, 600.0)),
            exit_on_close_request: false,
            ..Default::default()
        },
        antialiasing: true,
        ..iced::Settings::default()
    })
}
