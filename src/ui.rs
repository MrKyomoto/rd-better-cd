use std::{error::Error, fs};

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::{app::App, components::FileType};

pub fn render_ui(frame: &mut Frame, app: &App) -> Result<(), Box<dyn Error>> {
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

    let left_block = Block::new()
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));

    let mut left_list_items = Vec::<ListItem>::new();

    // NOTE: previous render logic without the consideration of out of range
    //
    // app.files.iter().for_each(|file| {
    //     left_list_items.push(ListItem::new(Line::from(match file.file_type {
    //         FileType::Dir => {
    //             if file.name == app.files[app.index].name {
    //                 Span::styled(
    //                     format!("{}/", file.name),
    //                     Style::default().fg(Color::LightRed),
    //                 )
    //             } else {
    //                 Span::styled(
    //                     format!("{}/", file.name),
    //                     Style::default().fg(Color::LightYellow),
    //                 )
    //             }
    //         }
    //         FileType::File => {
    //             if file.name == app.files[app.index].name {
    //                 Span::styled(
    //                     format!("{}", file.name),
    //                     Style::default().fg(Color::LightCyan),
    //                 )
    //             } else {
    //                 Span::styled(format!("{}", file.name), Style::default().fg(Color::White))
    //             }
    //         }
    //     })));
    // });

    let render_area = &middle_chunks[0];

    let visible_height = if render_area.height <= 2 {
        1 // 避免高度过小导致计算错误
    } else {
        (render_area.height - 2) as usize // 实际能显示的文件行数
    };

    let scroll_offset = if app.files.len() <= visible_height {
        0
    } else {
        // 当选中索引超过「可视区域底部」时，偏移量跟随索引移动
        // 可视区域底部 = 偏移量 + 可视高度 - 1 → 偏移量 = 索引 - 可视高度 + 1
        app.index.saturating_sub(visible_height - 1)
    };
    app.files
        .iter()
        .skip(scroll_offset)
        .take(visible_height)
        .for_each(|file| {
            left_list_items.push(ListItem::new(Line::from(match file.file_type {
                FileType::Dir => Span::styled(
                    format!("{}/", file.name),
                    Style::default().fg(Color::LightYellow),
                ),
                FileType::File => {
                    Span::styled(format!("{}", file.name), Style::default().fg(Color::White))
                }
            })));
        });
    let mut list_state = ListState::default();

    if !app.files.is_empty() {
        // 计算选中项在当前可视区域内的相对索引（用于高亮）
        let relative_index = app.index - scroll_offset;
        list_state.select(Some(relative_index));
    }

    let left_block_content = List::new(left_list_items)
        .highlight_symbol(">")
        .highlight_style(Style::default().fg(Color::LightRed))
        .block(left_block);

    frame.render_stateful_widget(left_block_content, middle_chunks[0], &mut list_state);
    // frame.render_widget(left_block_content, middle_chunks[0]);

    let right_block = Block::new()
        .title_top(
            Line::from(if !app.files.is_empty() {
                app.files[app.index].name.clone()
            } else {
                String::from("")
            })
            .style(Style::new().fg(Color::LightGreen)),
        )
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));

    let mut right_list_items = Vec::<ListItem>::new();

    app.sub_files.iter().for_each(|file| {
        right_list_items.push(ListItem::new(Line::from(Span::styled(
            format!("{}", file.name),
            Style::default().fg(match file.file_type {
                FileType::Dir => Color::LightYellow,
                FileType::File => Color::White,
            }),
        ))));
    });

    if !app.files.is_empty() && app.files[app.index].file_type == FileType::File {
        let path = app.current_dir.clone() + "/" + &app.files[app.index].name;
        let buffer = fs::read(path)?;
        let content =
            String::from_utf8(buffer).unwrap_or("Invalid UTF-8 content or Non-Text file".into());
        let right_block_file_content = Paragraph::new(content)
            .style(Style::default().fg(Color::White))
            .block(right_block);
        frame.render_widget(right_block_file_content, middle_chunks[1]);
    } else {
        let right_block_dir_content = List::new(right_list_items).block(right_block);
        frame.render_widget(right_block_dir_content, middle_chunks[1]);
    }

    let bottom_block = Block::new()
        .border_set(border::ROUNDED)
        .borders(Borders::ALL)
        .style(Style::new().fg(Color::Cyan));
    let bottom_instructions = Line::from(vec![
        " | ".into(),
        " <ESC/h> ".blue().bold(),
        "back".white(),
        " | ".into(),
        " <Enter/l> ".blue().bold(),
        "into".white(),
        " | ".into(),
        " <Space> ".blue().bold(),
        "choose".white(),
        " | ".into(),
        " <H> ".blue().bold(),
        "hide dot file".white(),
        " | ".into(),
        " <↑/k ↓/j> ".blue().bold(),
        "move ".white(),
        " | ".into(),
        " <q|Q> ".blue().bold(),
        "quit ".white(),
        " | ".into(),
    ]);

    let bottom_block_content = Paragraph::new(bottom_instructions.centered()).block(bottom_block);
    frame.render_widget(bottom_block_content, main_chunks[2]);

    Ok(())
}
