use ratzilla::{
    WebGl2Backend, WebRenderer,
    event::KeyCode,
    ratatui::{
        Terminal,
        layout::Rect,
        style::{Color, Style},
    },
};
use tachyonfx::{
    ColorSpace::Rgb,
    EffectTimer, Interpolation, Motion,
    fx::{self, EvolveSymbolSet},
    pattern::WavePattern,
    wave::{Modulator, Oscillator, WaveLayer},
};
#[derive(Debug, Clone, Default)]
pub struct App {}

impl App {
    pub fn handle_key_event(&mut self, key_code: KeyCode) {}

    pub fn run(mut self, terminal: Terminal<WebGl2Backend>) {
        let mut effect = get_wave_fx(Rect::new(0, 0, 1920, 1000));

        let mut cloned = self.clone();
        terminal.on_key_event(move |key_event| {
            cloned.handle_key_event(key_event.code);
        });

        terminal.draw_web(move |f| {
            self.render(f, &mut effect);
        });
    }
}

fn get_wave_fx(content_area: Rect) -> tachyonfx::Effect {
    let style = Style::default()
        .fg(Color::from_u32(0x32302F)) // content area bg
        .bg(Color::from_u32(0x1D2021)); // screen area bg

    let timer = EffectTimer::from_ms(750, Interpolation::Linear);

    let wave_a = Oscillator::sin(0.0, 0.3, -1.0);
    let wave_b = Oscillator::cos(0.2, 0.1, 0.3)
        .modulated_by(Modulator::sawtooth(-0.5, 0.2, -0.5).on_amplitude());

    let wave_layer = WaveLayer::new(wave_a).average(wave_b);
    let p = WavePattern::new(wave_layer);

    fx::prolong_start(
        500,
        fx::parallel(&[
            // organically evolving into content
            fx::evolve_into((EvolveSymbolSet::BlocksHorizontal, style), 1200)
                .with_pattern(p.clone()),
            // coalesce foreground symbols using the same pattern
            fx::coalesce(timer).with_pattern(p),
            // start faded to remove rough corners
            fx::fade_from(Color::from_u32(0x1D2021), Color::from_u32(0x1D2021), 500)
                .with_color_space(Rgb),
        ])
        .with_area(content_area),
    )
}
