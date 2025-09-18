use std::error::Error;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod config;
mod git;
mod tui;
mod uwu;

use config::Config;
use tui::{
    animation,
    events::{self, AnimationResult, PopupSelection},
    notifications::NotificationManager,
    terminal, ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    show_prank()?; // >:3c

    let config = Config::new().map_err(|e| e.to_string())?;

    if git::is_in_git_repo() {
        run_app(config)?;
    } else {
        ui::display_nothing_bruh()?;
    }

    Ok(())
}

fn run_app(config: Config) -> Result<(), Box<dyn Error>> {
    let (show_popup_tx, show_popup_rx) = mpsc::channel();
    let (reset_timer_tx, reset_timer_rx) = mpsc::channel();

    let loop_delay = config.loop_delay;
    thread::spawn(move || {
        git::git_watcher_loop(show_popup_tx, reset_timer_rx, loop_delay);
    });

    let mut terminal = terminal::setup_terminal()?;
    let mut notification_manager = NotificationManager::new(5);

    let mut frame_index = 0;
    let frame_duration = Duration::from_millis(20);
    let mut show_popup = false;
    let mut popup_selection = PopupSelection::Yes;

    loop {
        if show_popup_rx.try_recv().is_ok() {
            if config.autosave_mode {
                notification_manager.add_notif("Auto-committing changes...".to_string());
                git::perform_commit(&mut notification_manager)?;
                reset_timer_tx.send(()).ok();
            } else {
                show_popup = true;
            }
        }

        notification_manager.update(Duration::from_secs(15 * 60));

        terminal.draw(|f| {
            ui::draw_ui(
                f,
                frame_index,
                show_popup,
                &popup_selection,
                notification_manager.get_notifications(),
                config.loop_delay,
                config.shiggy_mode,
            );
        })?;

        if let Some(result) =
            events::handle_events(&mut show_popup, &mut popup_selection, &reset_timer_tx)?
        {
            match result {
                AnimationResult::Commit => {
                    git::perform_commit(&mut notification_manager)?;
                    show_popup = false;
                    reset_timer_tx.send(()).ok();
                }
                AnimationResult::Quit => {
                    break;
                }
            }
        }

        frame_index = (frame_index + 1) % animation::get_frame_count(config.shiggy_mode);
        thread::sleep(frame_duration);
    }

    terminal::restore_terminal(&mut terminal)?;
    Ok(())
}

fn show_prank() -> Result<(), Box<dyn Error>> {
    use crossterm::{cursor, execute, style, terminal};
    use std::io::stdout;

    eprintln!("
thread 'main' panicked at 'a critical error occurred: could not connect to the git daemon', src/main.rs:10:5");
    thread::sleep(Duration::from_secs(3));
    {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveUp(2),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            style::Print(
                "
>:D ha! just kidding!"
            )
        )?;
    }
    thread::sleep(Duration::from_secs(1));
    {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveToColumn(0),
            terminal::Clear(terminal::ClearType::CurrentLine)
        )?;
    }
    Ok(())
}
