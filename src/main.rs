use std::error::Error;
use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

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

        let mut frame_index = 0;
        let frame_duration = Duration::from_millis(60);
        let mut show_popup = false;
        let mut popup_selection = animation::PopupSelection::Yes;

        loop {
            if show_popup_rx.try_recv().is_ok() {
                show_popup = true;
            }

            terminal.draw(|f| {
                animation::draw_ui(f, frame_index, show_popup, &popup_selection);
            })?;

            if let Some(result) = animation::handle_events(&mut show_popup, &mut popup_selection, &reset_timer_tx)? {
                match result {
                    animation::AnimationResult::Commit => {
                        Command::new("git").arg("add").arg(".").status()?;
                        Command::new("git")
                            .arg("commit")
                            .arg("-m")
                            .arg("this commit is made by kwis uwu")
                            .status()?;
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
