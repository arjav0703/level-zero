use ratzilla::ratatui::widgets::{Paragraph, Wrap};

use super::*;

pub fn render_section_two(area: Rect, app: &mut App, frame: &mut Frame) {
    let vert_div = Layout::new(
        Direction::Vertical,
        [
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(65),
            Constraint::Percentage(5),
        ],
    )
    .split(area);

    frame.render_widget(get_big_faq(), vert_div[1]);

    let div = Layout::new(
        Direction::Horizontal,
        [
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ],
    )
    .split(vert_div[2]);

    let project_block = bordered_block().title("What can I build?");
    frame.render_widget(&project_block, div[0]);

    let inner_area = project_block.inner(div[0]);
    frame.render_widget(get_project_ideas_text(), inner_area);

    let faq_block = bordered_block().title("FAQ");
    frame.render_widget(&faq_block, div[1]);

    let inner_area = faq_block.inner(div[1]);
    frame.render_widget(get_faq_list(), inner_area);

    let ws_block = bordered_block().title("What do I get?");
    frame.render_widget(&ws_block, div[2]);

    let inner_area = ws_block.inner(div[2]);
    frame.render_widget(get_ws_items(), inner_area);
}

fn get_big_faq() -> impl Widget {
    BigText::builder()
        .alignment(HorizontalAlignment::Center)
        .pixel_size(PixelSize::Full)
        .style(Style::new().yellow())
        .lines(vec!["FAQ".light_red().into(), "~~~".red().into()])
        .build()
}

fn get_project_ideas_text() -> impl Widget {
    let content = [
        "• A HTTP server from scratch",
        "• A bittorrent client",
        "• Reinvent your favourite database (like redis, postgres, etc.)",
        "• A programming language!!!",
        "• An OS kernel (if you're brave enough)",
        "• Anything else you can think of! The possibilities are endless!",
    ]
    .join("\n\n");

    Paragraph::new(content)
        .style(Style::new().light_cyan().bold())
        .wrap(Wrap { trim: true })
}

fn get_faq_list() -> impl Widget {
    let items = [
        "Q: Who can participate?",
        "A: If you're a teenager between the age of 13 and 18, you are eligible to participate!\n",
        "Q: Do I need to be an expert to follow do this?",
        "A: Prior coding experience would definitely be useful as this is targeted towards intermediate/experienced programmers\n",
        "Q: Can I use AI?",
        "A: You may use tab completions but use of AI editors like cursor for code generation is not allowed. The goal is to learn by doing, and using AI to generate code would defeat that purpose.\n",
        "Q: Do I need to track hours with hackatime?",
        "A: Yes",
    ].join("\n\n");

    Paragraph::new(items)
        .style(Style::new().light_magenta().bold())
        .wrap(Wrap { trim: true })
}

fn get_ws_items() -> impl Widget {
    let items = [
        "-> RAM grant 🤑",
        "-> CPU grant ",
        "-> Laptop grant 💻",
        "-> Cool swag 👕",
        "-> More stuff to be announced!",
    ]
    .join("\n\n");

    Paragraph::new(items)
        .style(Style::new().light_green().bold())
        .wrap(Wrap { trim: true })
}

fn bordered_block() -> Block<'static> {
    Block::bordered().border_type(ratzilla::ratatui::widgets::BorderType::Rounded)
}
