use std::error::Error;
use std::process::Command;
use std::sync::mpsc;
use std::thread;

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
        let result = animation::run_animation(&mut terminal, show_popup_rx, reset_timer_tx)?;
        animation::restore_terminal(&mut terminal)?;

        if let animation::AnimationResult::Commit = result {
            Command::new("git").arg("add").arg(".").status()?;

            Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg("this commit is made by kwis uwu")
                .status()?;

            println!("Changes committed successfully!");
        }
    } else {
        animation::display_nothing_bruh()?;
    }

    Ok(())
}