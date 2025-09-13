use ansi_to_tui::IntoText;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    prelude::*,
    widgets::{Clear, Paragraph},
};
use std::error::Error;
use std::fs;
use std::io::{self, Stdout};
use std::time::Duration;
use std::time::Instant;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn run_animation(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let mut frame_files: Vec<String> = fs::read_dir("frames")?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(String::from))
            })
        })
        .collect();
    frame_files.sort();

    let frames: Vec<String> = frame_files
        .iter()
        .map(|file_name| fs::read_to_string(format!("frames/{}", file_name)).unwrap_or_default())
        .collect();

    let preprocessed_frames: Vec<Text> = frames
        .iter()
        .map(|frame_content| frame_content.as_bytes().into_text().unwrap())
        .collect();

    let mut frame_index = 0;
    let frame_duration = Duration::from_millis(50);

    loop {
        let start_time = Instant::now();

        terminal.draw(|f| {
            let area = f.area();
            f.render_widget(Clear, area);

            let ansi_text = &preprocessed_frames[frame_index];

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
                    Constraint::Length(80),
                    Constraint::Min(0),
                ])
                .split(vertical_layout[1]);

            let paragraph = Paragraph::new(ansi_text.clone()).alignment(Alignment::Center);
            f.render_widget(paragraph, horizontal_layout[1]);
        })?;

        if crossterm::event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }

        frame_index = (frame_index + 1) % preprocessed_frames.len();

        let elapsed_time = start_time.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }

    Ok(())
}

pub fn display_nothing_bruh(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    let nothing_bruh_content = fs::read_to_string("nothing_bruh.ans")?;
    let ansi_text = nothing_bruh_content.as_bytes().into_text().unwrap();

    terminal.draw(|f| {
        let area = f.area();
        f.render_widget(Clear, area); // Clear the entire area

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
                Constraint::Length(80),
                Constraint::Min(0),
            ])
            .split(vertical_layout[1]);

        let paragraph = Paragraph::new(ansi_text).alignment(Alignment::Center);
        f.render_widget(paragraph, horizontal_layout[1]);
    })?;

    // waiting for the user to press da button
    loop {
        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }
    }

    Ok(())
}