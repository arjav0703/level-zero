use super::*;

pub fn render_section_two(area: Rect, app: &mut App, frame: &mut Frame) {
    let vert_div = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ],
    )
    .split(area);

    frame.render_widget(get_big_faq(), vert_div[0]);
}

fn get_big_faq() -> impl Widget {
    BigText::builder()
        .alignment(HorizontalAlignment::Center)
        .pixel_size(PixelSize::Full)
        .style(Style::new().yellow())
        .lines(vec!["FAQ".light_red().into(), "~~~".red().into()])
        .build()
}
