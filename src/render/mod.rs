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
            [
                Constraint::Percentage(20),
                Constraint::Percentage(45),
                Constraint::Percentage(35),
            ],
        )
        .split(sections[0]);

        draw_gauges(frame, self, vert_div[0]);

        let horizontal_div = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Percentage(32),
                Constraint::Min(20),
                Constraint::Percentage(30),
            ],
        )
        .split(vert_div[1]);

        draw_pie_chart(frame, self, horizontal_div[0]);

        let hero_vertical = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ],
        )
        .split(horizontal_div[1]);

        frame.render_widget(hero_text(), hero_vertical[1]);

        draw_horizontal_barchart(frame, self, horizontal_div[2]);

        let info_block = Block::bordered().title("INFO");

        frame.render_widget(&info_block, vert_div[2]);

        let inner_area = info_block.inner(vert_div[2]);
        let info_vertical = Layout::new(
            Direction::Vertical,
            [
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ],
        )
        .split(inner_area);

        frame.render_widget(info_text(), info_vertical[1]);

        if effect.running() {
            frame.render_effect(effect, frame.area(), Duration::from_millis(100));
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

fn info_text() -> impl Widget {
    BigText::builder()
        .alignment(ratzilla::ratatui::layout::HorizontalAlignment::Center)
        .pixel_size(PixelSize::Quadrant)
        .style(Style::new().yellow())
        .lines(vec![
            "You ship -> A low level systems programming Project"
                .light_cyan()
                .into(),
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~"
                .cyan()
                .into(),
            "We Ship -> Cool computer hardware".light_green().into(),
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~".green().into(),
        ])
        .build()
}
