use ratzilla::ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::Style,
    style::Stylize,
};
use tui_big_text::{BigText, PixelSize};

use super::App;

impl App {
    pub fn render(&mut self, frame: &mut Frame) {
        let chunks = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(frame.area());

        let big_text = BigText::builder()
            .alignment(ratzilla::ratatui::layout::HorizontalAlignment::Center)
            .pixel_size(PixelSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                "level".light_red().into(),
                "zero".white().into(),
                "~~~~~".light_blue().into(),
            ])
            .build();
        frame.render_widget(big_text, chunks[0]);
    }
}
