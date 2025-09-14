use notify_rust::Notification;
use regex::Regex;
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

pub fn git_watcher_loop(show_popup_tx: Sender<()>, reset_timer_rx: Receiver<()>) {
    let mut uncommitted_changes_start_time: Option<Instant> = None;
    let mut previous_stats: Option<GitStats> = None;
    let mut last_notification_time = Instant::now();

    loop {
        if reset_timer_rx.try_recv().is_ok() {
            uncommitted_changes_start_time = None;
        }

        let current_stats = get_git_diff_stats();

        if let Some(stats) = &current_stats {
            if stats.total_changes > 0 {
                if uncommitted_changes_start_time.is_none() {
                    uncommitted_changes_start_time = Some(Instant::now());
                }

                if let Some(start_time) = uncommitted_changes_start_time {
                    if start_time.elapsed() > Duration::from_secs(20) {
                        if show_popup_tx.send(()).is_ok() {
                            uncommitted_changes_start_time = None;
                        } else {
                            break;
                        }
                    }
                }
            } else {
                uncommitted_changes_start_time = None;
            }
        } // aaaa commit aaaaaaa aaaaa aaaaaaaagaaaaaain aa

        // the notification is here, the other things is when the user really forgor to commit
        if last_notification_time.elapsed() > Duration::from_secs(20 * 60) {
            send_notification(current_stats, previous_stats);
            previous_stats = current_stats;
            last_notification_time = Instant::now();
        }

        thread::sleep(Duration::from_secs(60));
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
        .arg("--stat")
        .arg("HEAD")
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let re = Regex::new(r"(\d+)\s+insertions\(\+\),\s+(\d+)\s+deletions\(-\)").unwrap();

    let mut insertions = 0;
    let mut deletions = 0;

    if let Some(captures) = re.captures(stdout.as_ref()) {
        insertions = captures
            .get(1)
            .and_then(|m| m.as_str().parse().ok())
            .unwrap_or(0);
        deletions = captures
            .get(2)
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
    let body = match current_stats {
        Some(current) => {
            let mut message = String::new();

            if current.insertions == 0 && current.deletions == 0 {
                message.push_str("Nu-nu changes yet? owo Time to get to wowk!");
            } else {
                message.push_str(&format!(
                    "You have {} insewtions and {} dewetions, nyaa! ",
                    current.insertions, current.deletions
                ));

                if let Some(previous) = previous_stats {
                    if current.deletions > previous.deletions && current.deletions > 10 {
                        // deleting a lot for some reason
                        message.push_str(
                            "You'we on a deweting spwee! Cweaning up code wike a boss! ^w^",
                        );
                    } else if current.insertions > previous.insertions && current.insertions > 20 {
                        // becoming real ig
                        message.push_str(
                            "Wow, a sudden buwst of coding! Keep dat momentum going, uwu!",
                        );
                    } else if current.deletions > current.insertions && current.deletions > 10 {
                        // peak optimishashtiong :3
                        message.push_str("Optimizing wike a pwo! Wess is mowe, wight? :3");
                    } else if current.insertions > current.deletions && current.insertions > 20 {
                        // too much insertion
                        message.push_str("Wook at chu, coding away! Keep it up, nyaa!");
                    } else if current.total_changes > 30 && previous.total_changes < 5 {
                        // sudden tryharding
                        message.push_str("You've been quiet, but now you'we a coding machine! OwO");
                    }
                } else {
                    // peak optomozation
                    if current.deletions > current.insertions && current.deletions > 10 {
                        message.push_str("Optimizing wike a pwo! Wess is mowe, wight? :3");
                    } else if current.insertions > current.deletions && current.insertions > 20 {
                        message.push_str("Wook at chu, coding away! Keep it up, nyaa!");
                    }
                }
                // do not forget to commit ur changes !!
                if message.is_empty() || !message.contains("commit") {
                    message.push_str("Don't fowget to commit youw changes, pwease! ^w^");
                }
            }
            message
        }
        None => "Don't fowget to commit youw changes, pwease! ^w^".to_string(),
    };

    Notification::new()
        .summary("Kwis :3")
        .body(&body)
        .show()
        .unwrap();
}
