use crossterm::{cursor, execute, style, terminal};
use std::error::Error;
use std::io::stdout;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod animation;
mod git;

fn perform_commit(
    notification_manager: &mut animation::NotificationManager,
) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("add").arg(".").output()?;
    if output.status.success() {
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("this commit is made by kwis uwu")
            .output()?;
        if commit_output.status.success() {
            notification_manager.add_notif("Changes committed successfully!".to_string());
        } else {
            let error_message = String::from_utf8_lossy(&commit_output.stderr);
            notification_manager.add_notif(format!("Commit failed: {}", error_message));
        }
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        notification_manager.add_notif(format!("git add failed: {}", error_message));
    }
    Ok(())
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    let mut numeric_part = String::new();
    let mut unit_part = String::new();

    for c in s.chars() {
        if c.is_digit(10) || c == '.' {
            numeric_part.push(c);
        } else {
            unit_part.push(c);
        }
    }
    //q
    let value: f64 = numeric_part
        .parse()
        .map_err(|_| "Invalid number".to_string())?;
    let unit = unit_part.trim();

    match unit {
        "s" | "sec" => Ok(Duration::from_secs_f64(value)),
        "m" | "min" => Ok(Duration::from_secs_f64(value * 60.0)),
        "h" => Ok(Duration::from_secs_f64(value * 3600.0)),
        _ => Err("Invalid time unit".to_string()),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("\nthread 'main' panicked at 'a critical error occurred: could not connect to the git daemon', src/main.rs:10:5");
    thread::sleep(Duration::from_secs(3));
    {
        let mut stdout = stdout();
        execute!(
            stdout,
            cursor::MoveUp(2),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            style::Print("\n>:D ha! just kidding!")
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

    let args: Vec<String> = std::env::args().collect();
    let autosave_mode = args.contains(&"--autosave".to_string());
    let mut loop_delay = Duration::from_secs(15 * 60);

    if let Some(pos) = args.iter().position(|s| s == "-l" || s == "--loop-delay") {
        if let Some(value_str) = args.get(pos + 1) {
            loop_delay = parse_duration(value_str).unwrap_or(loop_delay);
        }
    }

    if git::is_in_git_repo() {
        let (show_popup_tx, show_popup_rx) = mpsc::channel();
        let (reset_timer_tx, reset_timer_rx) = mpsc::channel();

        thread::spawn(move || {
            git::git_watcher_loop(show_popup_tx, reset_timer_rx, loop_delay);
        });

        let mut terminal = animation::setup_terminal()?;
        let mut notification_manager = animation::NotificationManager::new(5);

        let mut frame_index = 0;
        let frame_duration = Duration::from_millis(60);
        let mut show_popup = false;
        let mut popup_selection = animation::PopupSelection::Yes;

        loop {
            if show_popup_rx.try_recv().is_ok() && autosave_mode {
                notification_manager.add_notif("Auto-committing changes...".to_string());
                perform_commit(&mut notification_manager)?;
                reset_timer_tx.send(()).ok();
            }

            notification_manager.update(Duration::from_secs(15 * 60));

            terminal.draw(|f| {
                animation::draw_ui(
                    f,
                    frame_index,
                    show_popup,
                    &popup_selection,
                    notification_manager.get_notifications(),
                );
            })?;

            if let Some(result) =
                animation::handle_events(&mut show_popup, &mut popup_selection, &reset_timer_tx)?
            {
                match result {
                    animation::AnimationResult::Commit => {
                        perform_commit(&mut notification_manager)?;
                        show_popup = false;
                        reset_timer_tx.send(()).ok();
                    }
                    animation::AnimationResult::Quit => {
                        break;
                    }
                }
            }

            frame_index = (frame_index + 1) % animation::get_frame_count();
            thread::sleep(frame_duration);
        }

        animation::restore_terminal(&mut terminal)?;
    } else {
        animation::display_nothing_bruh()?;
    }

    Ok(())
}
