use std::error::Error;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

mod animation;
mod git;

fn main() -> Result<(), Box<dyn Error>> {
    if git::is_in_git_repo() {
        let (show_popup_tx, show_popup_rx) = mpsc::channel();
        let (reset_timer_tx, reset_timer_rx) = mpsc::channel();

        thread::spawn(move || {
            git::git_watcher_loop(show_popup_tx, reset_timer_rx);
        });

        let mut terminal = animation::setup_terminal()?;
        let mut notification_manager = animation::NotificationManager::new(5);

        let mut frame_index = 0;
        let frame_duration = Duration::from_millis(60);
        let mut show_popup = false;
        let mut popup_selection = animation::PopupSelection::Yes;
        let mut popup_visible_since: Option<Instant> = None;
        let popup_duration = Duration::from_secs(10);

        loop {
            if show_popup_rx.try_recv().is_ok() {
                show_popup = true;
                popup_visible_since = Some(Instant::now());
                notification_manager.add_notif("Popup appeared!".to_string());
            }

            if show_popup {
                if let Some(since) = popup_visible_since {
                    if since.elapsed() > popup_duration {
                        show_popup = false;
                        popup_visible_since = None;
                        reset_timer_tx.send(()).ok();
                    }
                }
            }

            notification_manager.update(Duration::from_secs(10));

            terminal.draw(|f| {
                animation::draw_ui(
                    f,
                    frame_index,
                    show_popup,
                    &popup_selection,
                    notification_manager.get_notifications(),
                );
            })?;

            if let Some(result) = animation::handle_events(&mut show_popup, &mut popup_selection, &reset_timer_tx)? {
                match result {
                    animation::AnimationResult::Commit => {
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
                        show_popup = false;
                        popup_visible_since = None;
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
