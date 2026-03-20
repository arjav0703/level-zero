use super::*;

pub fn render_section_one(area: Rect, frame: &mut Frame, app: &mut App) {
    let vert_div = Layout::new(
        Direction::Vertical,
        [
            Constraint::Min(10),
            Constraint::Percentage(55),
            Constraint::Percentage(35),
        ],
    )
    .split(area);

    draw_gauges(frame, app, vert_div[0]);

    let horizontal_div = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(32),
            Constraint::Min(50),
            Constraint::Percentage(30),
        ],
    )
    .split(vert_div[1]);

    draw_pie_chart(frame, app, horizontal_div[0]);

    let hero_vertical = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(5),
            Constraint::Percentage(90),
            Constraint::Percentage(5),
        ],
    )
    .split(horizontal_div[1]);

    frame.render_widget(hero_text(), hero_vertical[1]);

    draw_horizontal_barchart(frame, app, horizontal_div[2]);

    let info_block = Block::bordered().title("INFO");

    frame.render_widget(&info_block, vert_div[2]);

    let inner_area = info_block.inner(vert_div[2]);
    // let info_vertical = Layout::new(
    //     Direction::Vertical,
    //     [
    //         Constraint::Percentage(15),
    //         Constraint::Percentage(70),
    //         Constraint::Percentage(15),
    //     ],
    // )
    // .split(inner_area);

    frame.render_widget(info_text(), inner_area);
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
            "YS -> A systems programming project".light_cyan().into(),
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~".cyan().into(),
            "WS -> Cool computer hardware".light_green().into(),
            "~~~~~~~~~~~~~~~~~~~~~~~~~~~~".green().into(),
        ])
        .build()
}
