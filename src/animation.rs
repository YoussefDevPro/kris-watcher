use ansi_to_tui::IntoText;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use lazy_static::lazy_static;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use std::error::Error;
use std::io::{self, Stdout};
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

pub enum AnimationResult {
    Commit,
    Quit,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PopupSelection {
    Yes,
    No,
}

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

lazy_static! {
    static ref PREPROCESSED_FRAMES: Vec<Text<'static>> = {
        let mut frames_content: Vec<&'static str> = Vec::new();
        frames_content.push(include_str!("../frames/frame_000.ans"));
        frames_content.push(include_str!("../frames/frame_001.ans"));
        frames_content.push(include_str!("../frames/frame_002.ans"));
        frames_content.push(include_str!("../frames/frame_003.ans"));
        frames_content.push(include_str!("../frames/frame_004.ans"));
        frames_content.push(include_str!("../frames/frame_005.ans"));
        frames_content.push(include_str!("../frames/frame_006.ans"));
        frames_content.push(include_str!("../frames/frame_007.ans"));
        frames_content.push(include_str!("../frames/frame_008.ans"));
        frames_content.push(include_str!("../frames/frame_009.ans"));
        frames_content.push(include_str!("../frames/frame_010.ans"));
        frames_content.push(include_str!("../frames/frame_011.ans"));
        frames_content.push(include_str!("../frames/frame_012.ans"));
        frames_content.push(include_str!("../frames/frame_013.ans"));
        frames_content.push(include_str!("../frames/frame_014.ans"));
        frames_content.push(include_str!("../frames/frame_015.ans"));
        frames_content.push(include_str!("../frames/frame_016.ans"));
        frames_content.push(include_str!("../frames/frame_017.ans"));
        frames_content.push(include_str!("../frames/frame_018.ans"));
        frames_content.push(include_str!("../frames/frame_019.ans"));
        frames_content.push(include_str!("../frames/frame_020.ans"));
        frames_content.push(include_str!("../frames/frame_021.ans"));
        frames_content.push(include_str!("../frames/frame_022.ans"));
        frames_content.push(include_str!("../frames/frame_023.ans"));
        frames_content.push(include_str!("../frames/frame_024.ans"));
        frames_content.push(include_str!("../frames/frame_025.ans"));
        frames_content.push(include_str!("../frames/frame_026.ans"));
        frames_content.push(include_str!("../frames/frame_027.ans"));
        frames_content.push(include_str!("../frames/frame_028.ans"));
        frames_content.push(include_str!("../frames/frame_029.ans"));
        frames_content.push(include_str!("../frames/frame_030.ans"));
        frames_content.push(include_str!("../frames/frame_031.ans"));
        frames_content.push(include_str!("../frames/frame_032.ans"));
        frames_content.push(include_str!("../frames/frame_033.ans"));
        frames_content.push(include_str!("../frames/frame_034.ans"));
        frames_content.push(include_str!("../frames/frame_035.ans"));
        frames_content.push(include_str!("../frames/frame_036.ans"));
        frames_content.push(include_str!("../frames/frame_037.ans"));
        frames_content.push(include_str!("../frames/frame_038.ans"));
        frames_content.push(include_str!("../frames/frame_039.ans"));
        frames_content.push(include_str!("../frames/frame_040.ans"));
        frames_content.push(include_str!("../frames/frame_041.ans"));
        frames_content.push(include_str!("../frames/frame_042.ans"));
        frames_content.push(include_str!("../frames/frame_043.ans"));
        frames_content.push(include_str!("../frames/frame_044.ans"));
        frames_content.push(include_str!("../frames/frame_045.ans"));
        frames_content.push(include_str!("../frames/frame_046.ans"));
        frames_content.push(include_str!("../frames/frame_047.ans"));
        frames_content.push(include_str!("../frames/frame_048.ans"));
        frames_content.push(include_str!("../frames/frame_049.ans"));
        frames_content.push(include_str!("../frames/frame_050.ans"));
        frames_content.push(include_str!("../frames/frame_051.ans"));
        frames_content.push(include_str!("../frames/frame_052.ans"));
        frames_content.push(include_str!("../frames/frame_053.ans"));
        frames_content.push(include_str!("../frames/frame_054.ans"));
        frames_content.push(include_str!("../frames/frame_055.ans"));
        frames_content.push(include_str!("../frames/frame_056.ans"));
        frames_content.push(include_str!("../frames/frame_057.ans"));
        frames_content.push(include_str!("../frames/frame_058.ans"));
        frames_content.push(include_str!("../frames/frame_059.ans"));
        frames_content.push(include_str!("../frames/frame_060.ans"));
        frames_content.push(include_str!("../frames/frame_061.ans"));
        frames_content.push(include_str!("../frames/frame_062.ans"));
        frames_content.push(include_str!("../frames/frame_063.ans"));
        frames_content.push(include_str!("../frames/frame_064.ans"));
        frames_content.push(include_str!("../frames/frame_065.ans"));
        frames_content.push(include_str!("../frames/frame_066.ans"));
        frames_content.push(include_str!("../frames/frame_067.ans"));
        frames_content.push(include_str!("../frames/frame_068.ans"));
        frames_content.push(include_str!("../frames/frame_069.ans"));
        frames_content.push(include_str!("../frames/frame_070.ans"));
        frames_content.push(include_str!("../frames/frame_071.ans"));
        frames_content.push(include_str!("../frames/frame_072.ans"));
        frames_content.push(include_str!("../frames/frame_073.ans"));
        frames_content.push(include_str!("../frames/frame_074.ans"));
        frames_content.push(include_str!("../frames/frame_075.ans"));
        frames_content.push(include_str!("../frames/frame_076.ans"));
        frames_content.push(include_str!("../frames/frame_077.ans"));
        frames_content.push(include_str!("../frames/frame_078.ans"));
        frames_content.push(include_str!("../frames/frame_079.ans"));
        frames_content.push(include_str!("../frames/frame_080.ans"));
        frames_content.push(include_str!("../frames/frame_081.ans"));
        frames_content.push(include_str!("../frames/frame_082.ans"));
        frames_content.push(include_str!("../frames/frame_083.ans"));
        frames_content.push(include_str!("../frames/frame_084.ans"));
        frames_content.push(include_str!("../frames/frame_085.ans"));
        frames_content.push(include_str!("../frames/frame_086.ans"));
        frames_content.push(include_str!("../frames/frame_087.ans"));
        frames_content.push(include_str!("../frames/frame_088.ans"));
        frames_content.push(include_str!("../frames/frame_089.ans"));
        frames_content.push(include_str!("../frames/frame_090.ans"));
        frames_content.push(include_str!("../frames/frame_091.ans"));
        frames_content.push(include_str!("../frames/frame_092.ans"));
        frames_content.push(include_str!("../frames/frame_093.ans"));
        frames_content.push(include_str!("../frames/frame_094.ans"));
        frames_content.push(include_str!("../frames/frame_095.ans"));
        frames_content.push(include_str!("../frames/frame_096.ans"));
        frames_content.push(include_str!("../frames/frame_097.ans"));
        frames_content.push(include_str!("../frames/frame_098.ans"));
        frames_content.push(include_str!("../frames/frame_099.ans"));
        frames_content.push(include_str!("../frames/frame_100.ans"));
        frames_content.push(include_str!("../frames/frame_101.ans"));
        frames_content.push(include_str!("../frames/frame_102.ans"));
        frames_content.push(include_str!("../frames/frame_103.ans"));
        frames_content.push(include_str!("../frames/frame_104.ans"));
        frames_content.push(include_str!("../frames/frame_105.ans"));
        frames_content.push(include_str!("../frames/frame_106.ans"));
        frames_content.push(include_str!("../frames/frame_107.ans"));
        frames_content.push(include_str!("../frames/frame_108.ans"));
        frames_content.push(include_str!("../frames/frame_109.ans"));
        frames_content.push(include_str!("../frames/frame_110.ans"));
        frames_content.push(include_str!("../frames/frame_111.ans"));
        frames_content.push(include_str!("../frames/frame_112.ans"));
        frames_content.push(include_str!("../frames/frame_113.ans"));
        frames_content.push(include_str!("../frames/frame_114.ans"));
        frames_content.push(include_str!("../frames/frame_115.ans"));
        frames_content.push(include_str!("../frames/frame_116.ans"));
        frames_content.push(include_str!("../frames/frame_117.ans"));
        frames_content.push(include_str!("../frames/frame_118.ans"));
        frames_content.push(include_str!("../frames/frame_119.ans"));
        frames_content.push(include_str!("../frames/frame_120.ans"));
        frames_content.push(include_str!("../frames/frame_121.ans"));
        frames_content.push(include_str!("../frames/frame_122.ans"));
        frames_content.push(include_str!("../frames/frame_123.ans"));
        frames_content.push(include_str!("../frames/frame_124.ans"));
        frames_content.push(include_str!("../frames/frame_125.ans"));
        frames_content.push(include_str!("../frames/frame_126.ans"));
        frames_content.push(include_str!("../frames/frame_127.ans"));
        frames_content.push(include_str!("../frames/frame_128.ans"));
        frames_content.push(include_str!("../frames/frame_129.ans"));
        frames_content.push(include_str!("../frames/frame_130.ans"));
        frames_content.push(include_str!("../frames/frame_131.ans"));
        frames_content.push(include_str!("../frames/frame_132.ans"));
        frames_content.push(include_str!("../frames/frame_133.ans"));
        frames_content.push(include_str!("../frames/frame_134.ans"));
        frames_content.push(include_str!("../frames/frame_135.ans"));
        frames_content.push(include_str!("../frames/frame_136.ans"));
        frames_content.push(include_str!("../frames/frame_137.ans"));
        frames_content.push(include_str!("../frames/frame_138.ans"));
        frames_content.push(include_str!("../frames/frame_139.ans"));
        frames_content.push(include_str!("../frames/frame_140.ans"));
        frames_content.push(include_str!("../frames/frame_141.ans"));
        frames_content.push(include_str!("../frames/frame_142.ans"));
        frames_content.push(include_str!("../frames/frame_143.ans"));
        frames_content.push(include_str!("../frames/frame_144.ans"));
        frames_content.push(include_str!("../frames/frame_145.ans"));
        frames_content.push(include_str!("../frames/frame_146.ans"));
        frames_content.push(include_str!("../frames/frame_147.ans"));
        frames_content.push(include_str!("../frames/frame_148.ans"));
        frames_content.push(include_str!("../frames/frame_149.ans"));
        frames_content.push(include_str!("../frames/frame_150.ans"));
        frames_content.push(include_str!("../frames/frame_151.ans"));
        frames_content.push(include_str!("../frames/frame_152.ans"));
        frames_content.push(include_str!("../frames/frame_153.ans"));
        frames_content.push(include_str!("../frames/frame_154.ans"));
        frames_content.push(include_str!("../frames/frame_155.ans"));
        frames_content.push(include_str!("../frames/frame_156.ans"));
        frames_content.push(include_str!("../frames/frame_157.ans"));
        frames_content.push(include_str!("../frames/frame_158.ans"));
        frames_content.push(include_str!("../frames/frame_159.ans"));
        frames_content.push(include_str!("../frames/frame_160.ans"));
        frames_content.push(include_str!("../frames/frame_161.ans"));
        frames_content.push(include_str!("../frames/frame_162.ans"));
        frames_content.push(include_str!("../frames/frame_163.ans"));
        frames_content.push(include_str!("../frames/frame_164.ans"));
        frames_content.push(include_str!("../frames/frame_165.ans"));
        frames_content.push(include_str!("../frames/frame_166.ans"));
        frames_content.push(include_str!("../frames/frame_167.ans"));
        frames_content.push(include_str!("../frames/frame_168.ans"));
        frames_content.push(include_str!("../frames/frame_169.ans"));
        frames_content.push(include_str!("../frames/frame_170.ans"));
        frames_content.push(include_str!("../frames/frame_171.ans"));
        frames_content.push(include_str!("../frames/frame_172.ans"));
        frames_content.push(include_str!("../frames/frame_173.ans"));
        frames_content.push(include_str!("../frames/frame_174.ans"));
        frames_content.push(include_str!("../frames/frame_175.ans"));
        frames_content.push(include_str!("../frames/frame_176.ans"));
        frames_content.push(include_str!("../frames/frame_177.ans"));
        frames_content.push(include_str!("../frames/frame_178.ans"));
        frames_content.push(include_str!("../frames/frame_179.ans"));
        frames_content.push(include_str!("../frames/frame_180.ans"));
        frames_content.push(include_str!("../frames/frame_181.ans"));
        frames_content.push(include_str!("../frames/frame_182.ans"));
        frames_content.push(include_str!("../frames/frame_183.ans"));
        frames_content.push(include_str!("../frames/frame_184.ans"));
        frames_content.push(include_str!("../frames/frame_185.ans"));
        frames_content.push(include_str!("../frames/frame_186.ans"));

        frames_content
            .into_iter()
            .map(|frame_content| frame_content.as_bytes().into_text().unwrap())
            .collect()
    };
}

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

fn draw_commit_popup(f: &mut Frame, selected: &PopupSelection) {
    let area = f.area();
    let popup_area = centered_rect(50, 20, area);

    f.render_widget(Clear, popup_area);

    let popup_block = Block::default() 
        .title(" Uncommitted Changes ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let inner_area = popup_block.inner(popup_area);
    f.render_widget(popup_block, popup_area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)].as_ref())
        .split(inner_area);

    let question = 
        Paragraph::new("You have uncommitted changes for over an hour. Do you want to commit them?")
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
    f.render_widget(question, chunks[0]);

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .horizontal_margin(1)
        .split(chunks[1]);

    let yes_style = if *selected == PopupSelection::Yes {
        Style::default().fg(Color::White).bg(Color::Green)
    } else {
        Style::default().fg(Color::Black).bg(Color::DarkGray)
    };
    let yes_button = Paragraph::new("Yes")
        .style(yes_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(yes_button, button_chunks[0]);

    let no_style = if *selected == PopupSelection::No {
        Style::default().fg(Color::White).bg(Color::Red)
    } else {
        Style::default().fg(Color::Black).bg(Color::DarkGray)
    };
    let no_button = Paragraph::new("No")
        .style(no_style)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(no_button, button_chunks[1]);
}

pub fn get_frame_count() -> usize {
    PREPROCESSED_FRAMES.len()
}

pub fn draw_ui(f: &mut Frame, frame_index: usize, show_popup: bool, popup_selection: &PopupSelection) {
    let area = f.area();
    f.render_widget(Clear, area);

    let ansi_text = &PREPROCESSED_FRAMES[frame_index];

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

    if show_popup {
        draw_commit_popup(f, popup_selection);
    }
}

pub fn handle_events(
    show_popup: &mut bool,
    popup_selection: &mut PopupSelection, 
    reset_timer_tx: &Sender<()>, 
) -> Result<Option<AnimationResult>, Box<dyn Error>> {
    if crossterm::event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if *show_popup {
                match key.code {
                    KeyCode::Left | KeyCode::Char('h') => *popup_selection = PopupSelection::Yes,
                    KeyCode::Right | KeyCode::Char('l') => *popup_selection = PopupSelection::No,
                    KeyCode::Enter => {
                        match popup_selection {
                            PopupSelection::Yes => return Ok(Some(AnimationResult::Commit)),
                            PopupSelection::No => {
                                *show_popup = false;
                                reset_timer_tx.send(()).ok();
                            }
                        }
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        *show_popup = false;
                        reset_timer_tx.send(()).ok();
                    }
                    _ => {}
                }
            } else if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                return Ok(Some(AnimationResult::Quit));
            }
        }
    }
    Ok(None)
}

pub fn display_nothing_bruh() -> Result<(), Box<dyn Error>> {
    let nothing_bruh_content = include_str!("../nothing_bruh.ans").to_string();

    #[cfg(windows)]
    {
        nothing_bruh_content = nothing_bruh_content.replace("\n", "\r\n");
    }

    println!("dude, wth do u want me to watch if there is no repo, i need my friend git to tell me the freaking changes u have made");
    print!("{}", nothing_bruh_content);
    Ok(())
}