use crate::tui::animation;
use crate::tui::events::PopupSelection;
use crate::tui::notifications::Notification;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
};
use std::collections::VecDeque;
use std::error::Error;
use std::time::Duration;

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    if total_seconds == 1 {
        "1 second".to_string()
    } else if total_seconds < 60 {
        format!("{} seconds", total_seconds)
    } else if total_seconds < 120 {
        "1 minute".to_string()
    } else if total_seconds < 3600 {
        format!("{} minutes", total_seconds / 60)
    } else if total_seconds < 7200 {
        "1 hour".to_string()
    } else {
        format!("{} hours", total_seconds / 3600)
    }
}

fn draw_commit_popup(f: &mut Frame, selected: &PopupSelection, loop_delay: Duration) {
    let area = f.area();
    let popup_area = centered_rect(25, 15, area);

    f.render_widget(Clear, popup_area);

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Rgb(255, 255, 255)));

    let inner_area = popup_block.inner(popup_area);
    f.render_widget(popup_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(inner_area);

    let question_text = format!(
        "ayo! you have uncommitted changes for over {}. Do ya want to commit them?",
        format_duration(loop_delay)
    );
    let question = Paragraph::new(question_text)
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Rgb(255, 255, 255)));
    f.render_widget(question, chunks[0]);

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .horizontal_margin(1)
        .split(chunks[1]);

    let yes_style = if *selected == PopupSelection::Yes {
        Style::default().fg(Color::Rgb(255, 255, 255))
    } else {
        Style::default().fg(Color::Rgb(128, 128, 128))
    };
    let yes_button = Paragraph::new("Yes")
        .style(yes_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(yes_button, button_chunks[0]);

    let no_style = if *selected == PopupSelection::No {
        Style::default().fg(Color::Rgb(255, 255, 255))
    } else {
        Style::default().fg(Color::Rgb(128, 128, 128))
    };
    let no_button = Paragraph::new("No")
        .style(no_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(no_button, button_chunks[1]);
}

fn draw_notifications(f: &mut Frame, notifs: &VecDeque<Notification>) {
    if notifs.is_empty() {
        return;
    }

    let notif_area = {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(notifs.len() as u16 + 2),
                Constraint::Min(0),
            ])
            .split(f.area());
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Min(0)])
            .split(chunks[0])[0]
    };

    let notif_block = Block::default()
        .title("Notifications")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let text: Vec<Line> = notifs
        .iter()
        .map(|n| Line::from(n.message.as_str()))
        .collect();
    let paragraph = Paragraph::new(text).block(notif_block);

    f.render_widget(paragraph, notif_area);
}

pub fn draw_ui(
    f: &mut Frame,
    frame_index: usize,
    show_popup: bool,
    popup_selection: &PopupSelection,
    notifications: &VecDeque<Notification>,
    loop_delay: Duration,
    shiggy_mode: bool,
) {
    let area = f.area();
    f.render_widget(Clear, area);

    let ansi_text = animation::get_frame(frame_index, shiggy_mode);
    let frame_width = ansi_text.width() as u16;

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(40),
            Constraint::Min(0),
        ])
        .split(area);

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(frame_width),
            Constraint::Min(0),
        ])
        .split(vertical_layout[1]);

    let paragraph = Paragraph::new(ansi_text).block(Block::default());
    f.render_widget(paragraph, horizontal_layout[1]);

    if show_popup {
        draw_commit_popup(f, popup_selection, loop_delay);
    }

    draw_notifications(f, notifications);
}

pub fn display_nothing_bruh() -> Result<(), Box<dyn Error>> {
    let nothing_bruh_content = include_str!("../../nothing_bruh.ans").to_string();

    println!();
    println!("dude, wth do u want me to watch if there is no repo, i need my friend git to tell me the freaking changes u have made");
    print!("{}", nothing_bruh_content);
    Ok(())
}
