use crate::tui::notifications::NotificationManager;
use crate::uwu;
use notify_rust::Notification;
use regex::Regex;
use std::error::Error;
use std::process::Command;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub struct GitStats {
    pub insertions: u32,
    pub deletions: u32,
    pub total_changes: u32,
}

pub fn perform_commit(
    notification_manager: &mut NotificationManager,
) -> Result<(), Box<dyn Error>> {
    let output = Command::new("git").arg("add").arg(".").output()?;
    if output.status.success() {
        let commit_output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(uwu::get_commit_message())
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

pub fn git_watcher_loop(
    show_popup_tx: Sender<()>,
    _reset_timer_rx: Receiver<()>,
    loop_delay: Duration,
) {
    let mut previous_stats: Option<GitStats> = None;
    let mut last_notification_time = Instant::now();

    loop {
        let current_stats = get_git_diff_stats();

        if last_notification_time.elapsed() > loop_delay {
            send_notification(current_stats, previous_stats);
            previous_stats = current_stats;
            last_notification_time = Instant::now();
        }

        if let Some(stats) = &current_stats {
            if stats.total_changes > 0 {
                if show_popup_tx.send(()).is_err() {
                    break;
                }
            }
        }

        thread::sleep(loop_delay);
    }
}

pub fn is_in_git_repo() -> bool {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--is-inside-work-tree")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.trim() == "true";
            }
            false
        }
        Err(_) => false,
    }
}

pub fn get_git_diff_stats() -> Option<GitStats> {
    let output = Command::new("git")
        .arg("diff")
        .arg("--shortstat")
        .arg("HEAD")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(
        r"(\d+)?(?: file)s? changed(?:, (\d+)? insertions?\(\+\))?(?:, (\d+)? deletions?\(-\))?",
    )
    .unwrap();

    let mut insertions = 0;
    let mut deletions = 0;

    if let Some(captures) = re.captures(stdout.as_ref()) {
        insertions = captures
            .get(2)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        deletions = captures
            .get(3)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
    }

    let total_changes = insertions + deletions;

    Some(GitStats {
        insertions,
        deletions,
        total_changes,
    })
}

fn send_notification(current_stats: Option<GitStats>, previous_stats: Option<GitStats>) {
    let body = uwu::get_notification_body(current_stats, previous_stats);

    Notification::new()
        .summary("Kwis :3")
        .body(&body)
        .show()
        .unwrap();
}
