use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, components::FileType};

pub fn render_ui(frame: &mut Frame, app: &App) {
    // TODO: 根据 index 来高亮选中的 sub file
    // TODO: 如果sub file 并非 dir 则在侧边栏加载该文本文件的预览
    // TODO: 如果屏幕放不下该如何做? 需要做滑动模块 but how?
    let main_chunks = Layout::default()
        .direction(ratatui::layout::Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    // NOTE: 1st part
    let top_block = Block::new()
        .title_top("Current root dir")
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));
    let top_block_content = Paragraph::new(
        Text::from(app.current_dir.clone())
            .centered()
            .style(Style::new().fg(Color::Green))
            .bold(),
    )
    .block(top_block);

    frame.render_widget(top_block_content, main_chunks[0]);

    // NOTE: 2nd part
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[1]);

    let right_block = Block::new()
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));

    let mut right_list_items = Vec::<ListItem>::new();

    app.files.iter().for_each(|file| {
        right_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", file.name),
            Style::default().fg(match file.file_type {
                FileType::Dir => Color::LightYellow,
                FileType::File => Color::White,
            }),
        ))));
    });

    let right_block_content = List::new(right_list_items).block(right_block);
    frame.render_widget(right_block_content, middle_chunks[0]);

    let left_block = Block::new()
        .title_top(
            Line::from(app.files[app.index].name.clone()).style(Style::new().fg(Color::LightGreen)),
        )
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));

    let mut left_list_items = Vec::<ListItem>::new();

    app.sub_files.iter().for_each(|file| {
        left_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", file.name),
            Style::default().fg(match file.file_type {
                FileType::Dir => Color::LightYellow,
                FileType::File => Color::White,
            }),
        ))));
    });

    let left_block_content = List::new(left_list_items).block(left_block);
    frame.render_widget(left_block_content, middle_chunks[1]);

    let bottom_block = Block::new()
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));
    let bottom_instructions = Line::from(vec![
        " | ".into(),
        " <ESC> ".blue().bold(),
        "back".white(),
        " | ".into(),
        " <Enter> ".blue().bold(),
        "into".white(),
        " | ".into(),
        " <Space> ".blue().bold(),
        "choose".white(),
        " | ".into(),
        " <h/H> ".blue().bold(),
        "hide dot file".white(),
        " | ".into(),
        " <↑/k ↓/j> ".blue().bold(),
        "move ".white(),
        " | ".into(),
    ]);

    let bottom_block_content = Paragraph::new(bottom_instructions.centered()).block(bottom_block);
    frame.render_widget(bottom_block_content, main_chunks[2]);
}
