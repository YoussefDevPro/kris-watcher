use std::error::Error;
use std::thread;

mod animation;
mod git;

fn main() -> Result<(), Box<dyn Error>> {
    if git::is_in_git_repo() {
        thread::spawn(git::git_watcher_loop);
    } else {
        let mut terminal = animation::setup_terminal()?;
        animation::display_nothing_bruh(&mut terminal)?;
        animation::restore_terminal(&mut terminal)?;
        return Ok(());
    }

    let mut terminal = animation::setup_terminal()?;
    animation::run_animation(&mut terminal)?;
    animation::restore_terminal(&mut terminal)?;
    Ok(())
}
