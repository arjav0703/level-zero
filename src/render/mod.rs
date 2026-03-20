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
mod section_one;
use section_one::render_section_one;

mod section_two;
use section_two::render_section_two;

impl App {
    pub fn render(&mut self, frame: &mut Frame, effect: &mut Effect) {
        Clear.render(frame.area(), frame.buffer_mut());

        let sections = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .split(frame.area());

        render_section_one(sections[0], frame, self);
        render_section_two(sections[1], self, frame);

        if effect.running() {
            frame.render_effect(effect, frame.area(), Duration::from_millis(100));
        }
    }
}
