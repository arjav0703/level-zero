use super::*;

pub fn draw_gauges(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(2),
    ])
    .margin(2)
    .split(area);
    let block = Block::bordered()
        .title("Cooking a new project")
        .border_type(ratzilla::ratatui::widgets::BorderType::Rounded);
    frame.render_widget(block, area);

    let label = format!("{:.2}%", app.progress1 * 100.0);
    let gauge = Gauge::default()
        .block(Block::new().title("Pre Processing:"))
        .gauge_style(
            Style::default()
                .fg(Color::LightYellow)
                .bg(Color::Black)
                .add_modifier(Modifier::ITALIC | Modifier::BOLD),
        )
        .use_unicode(true)
        .label(label)
        .ratio(app.progress1);
    frame.render_widget(gauge, chunks[0]);

    let sparkline = Sparkline::default()
        .block(Block::new().title("Compiling:"))
        .style(Style::default().fg(Color::LightGreen))
        .data(&app.sparkline.points)
        .bar_set(symbols::bar::NINE_LEVELS);
    frame.render_widget(sparkline, chunks[1]);

    let line_gauge = LineGauge::default()
        .block(Block::new().title("Linking Binary:"))
        .filled_style(Style::default().fg(Color::LightMagenta))
        .filled_symbol(symbols::line::THICK.horizontal)
        .ratio(app.progress2);
    frame.render_widget(line_gauge, chunks[2]);
}

use tui_piechart::{PieChart, PieSlice};

pub fn draw_pie_chart(frame: &mut Frame, app: &mut App, area: Rect) {
    let slices = vec![
        PieSlice::new("Test Failures", app.pie_chart_data.val1, Color::LightRed),
        PieSlice::new("Warning", app.pie_chart_data.val2, Color::LightBlue),
        PieSlice::new("Test Passed", app.pie_chart_data.val3, Color::LightGreen),
    ];

    let piechart = PieChart::new(slices);

    frame.render_widget(piechart, area);

    app.pie_chart_data.val1 = (app.pie_chart_data.val1 + 0.5) % 100.0;
    app.pie_chart_data.val2 = (app.pie_chart_data.val2 + 0.3) % 100.0;
    app.pie_chart_data.val3 = (app.pie_chart_data.val3 + 0.2) % 100.0;
}

// use tachyonfx::{
//     ColorSpace::Rgb,
//     EffectTimer, Interpolation, Motion,
//     fx::{self, EvolveSymbolSet},
//     pattern::WavePattern,
//     wave::{Modulator, Oscillator, WaveLayer},
// };
//
use tachyonfx::fx;

pub fn get_wave_fx(content_area: Rect) -> tachyonfx::Effect {
    // let style = Style::default()
    //     .fg(Color::from_u32(0x32302F)) // content area bg
    //     .bg(Color::from_u32(0x1D2021)); // screen area bg
    //
    // let timer = EffectTimer::from_ms(750, Interpolation::Linear);
    //
    // let wave_a = Oscillator::sin(0.0, 0.3, -1.0);
    // let wave_b = Oscillator::cos(0.2, 0.1, 0.3)
    //     .modulated_by(Modulator::sawtooth(-0.5, 0.2, -0.5).on_amplitude());
    //
    // let wave_layer = WaveLayer::new(wave_a).average(wave_b);
    // let p = WavePattern::new(wave_layer);
    //
    // fx::prolong_start(
    //     500,
    //     fx::parallel(&[
    //         // organically evolving into content
    //         fx::evolve_into((EvolveSymbolSet::BlocksHorizontal, style), 1200)
    //             .with_pattern(p.clone()),
    //         // coalesce foreground symbols using the same pattern
    //         fx::coalesce(timer).with_pattern(p),
    //         // start faded to remove rough corners
    //         fx::fade_from(Color::from_u32(0x1D2021), Color::from_u32(0x1D2021), 500)
    //             .with_color_space(Rgb),
    //     ])
    //     .with_area(content_area),
    // )
    fx::explode(10.0, 3.0, 800).with_area(content_area)
}
