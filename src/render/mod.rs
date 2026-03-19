use super::App;
use ratzilla::ratatui::{
    Frame,
    // layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Style, Stylize},
    widgets::{Block, Clear, Gauge, LineGauge, Sparkline},
};
use tachyonfx::{Duration, Effect, EffectRenderer};
use tui_big_text::{BigText, PixelSize};

mod effects;
pub use effects::*;

impl App {
    pub fn render(&mut self, frame: &mut Frame, effect: &mut Effect) {
        Clear.render(frame.area(), frame.buffer_mut());

        let sections = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(frame.area());

        let vert_div = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(20), Constraint::Percentage(50)],
        )
        .split(sections[0]);

        draw_gauges(frame, self, vert_div[0]);

        let horizontal_div = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ],
        )
        .split(vert_div[1]);

        draw_pie_chart(frame, self, horizontal_div[0]);

        frame.render_widget(hero_text(), horizontal_div[1]);

        draw_horizontal_barchart(frame, self, horizontal_div[2]);

        if effect.running() {
            frame.render_effect(effect, vert_div[0], Duration::from_millis(100));
        }
    }
}

fn hero_text() -> impl Widget {
    BigText::builder()
        .alignment(ratzilla::ratatui::layout::HorizontalAlignment::Center)
        .pixel_size(PixelSize::Full)
        .style(Style::new().blue())
        .lines(vec![
            "level".light_red().into(),
            "zero".white().into(),
            "~~~~~".light_blue().into(),
        ])
        .build()
}
