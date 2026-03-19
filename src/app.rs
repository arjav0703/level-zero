use ratzilla::{
    WebGl2Backend, WebRenderer,
    event::KeyCode,
    ratatui::{Terminal, layout::Rect},
};

mod utils;

#[derive(Clone)]
pub struct App {
    /// pre-processing
    pub progress1: f64,

    // compiling
    pub progress2: f64,

    pub sparkline: Signal<RandomSignal>,

    pub pie_chart_data: PieChartData,
    pub bar_chart_data: BarChartData,
}

impl Default for App {
    fn default() -> Self {
        Self {
            progress1: 0.0,
            progress2: 0.0,
            sparkline: Signal {
                source: RandomSignal::new(0, 100),
                points: Vec::new(),
                tick_rate: 5,
            },
            pie_chart_data: PieChartData::default(),
            bar_chart_data: BarChartData::default(),
        }
    }
}

impl App {
    pub fn on_tick(&mut self) {
        if self.progress2 >= 0.9 {
            self.progress1 = 0.0;
            self.progress2 = 0.0;
        }

        self.progress1 += 0.01;
        if self.progress1 >= 1.0 {
            self.progress1 = 1.0;
            self.progress2 += 0.004;
        }

        self.bar_chart_data.on_tick();

        if self.progress1 >= 1.0 {
            // Ensure we have enough data points to fill the screen width
            if self.sparkline.points.len() < 200 {
                self.sparkline
                    .points
                    .extend(self.sparkline.source.by_ref().take(10));
            } else {
                // Normal scrolling behavior
                self.sparkline.on_tick();
            }
        }
    }
}

use crate::{
    app::utils::{BarChartData, PieChartData, RandomSignal, Signal},
    render::get_wave_fx,
};

impl App {
    pub fn handle_key_event(&mut self, _key_code: KeyCode) {}

    pub fn run(mut self, terminal: Terminal<WebGl2Backend>) {
        let mut effect = get_wave_fx(Rect::new(0, 0, 1920, 1000));

        let mut cloned = self.clone();
        terminal.on_key_event(move |key_event| {
            cloned.handle_key_event(key_event.code);
        });

        terminal.draw_web(move |f| {
            self.on_tick();
            self.render(f, &mut effect);
        });
    }
}
