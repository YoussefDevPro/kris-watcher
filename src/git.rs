use crate::tui::notifications::NotificationManager;
use crate::uwu;
use anyhow::Result;
use git2::{Repository, StatusOptions}; // thx for @skyevg to tell me that there is a crate to do
                                       // this instead of using cmds
use notify_rust::Notification;
use rodio::{OutputStream, Sink};
use std::io::Cursor;
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
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(".")?;
    let mut index = repo.index()?;
    index.add_all(&["."], git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let oid = index.write_tree()?;
    let parent_commit = find_last_commit(&repo)?;
    let tree = repo.find_tree(oid)?;

    let signature = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &uwu::get_commit_message(),
        &tree,
        &[&parent_commit],
    )?;

    notification_manager.add_notif("Changes committed successfully!".to_string());
    Ok(())
}

fn find_last_commit(repo: &Repository) -> Result<git2::Commit<'_>, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

pub fn git_watcher_loop(
    show_popup_tx: Sender<()>,
    _reset_timer_rx: Receiver<()>,
    loop_delay: Duration,
    audio_alert_mode: bool,
    _shiggy_mode: bool,
) {
    let mut previous_stats: Option<GitStats> = None;
    let mut last_notification_time = Instant::now();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop {
        let current_stats = get_git_diff_stats().ok().flatten();

        if last_notification_time.elapsed() > loop_delay {
            send_notification(current_stats, previous_stats);
            previous_stats = current_stats;
            last_notification_time = Instant::now();
        }

        if let Some(stats) = &current_stats {
            if stats.total_changes > 0 {
                if audio_alert_mode && sink.empty() {
                    let mp3_data = include_bytes!("../sounds/yes.mp3");
                    if let Ok(source) = rodio::Decoder::new(Cursor::new(mp3_data)) {
                        sink.append(source);
                    }
                }
                if show_popup_tx.send(()).is_err() {
                    break;
                }
            }
        }

        thread::sleep(loop_delay);
    }
}

pub fn is_in_git_repo() -> bool {
    Repository::open(".").is_ok()
}

pub fn get_git_diff_stats() -> Result<Option<GitStats>> {
    let repo = Repository::open(".")?;
    let mut opts = StatusOptions::new();
    opts.include_untracked(true).recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut opts))?;
    let mut total_changes = 0;
    for entry in statuses.iter() {
        total_changes += match entry.status() {
            s if s.is_wt_new()
                || s.is_wt_modified()
                || s.is_wt_deleted()
                || s.is_wt_renamed()
                || s.is_wt_typechange()
                || s.is_index_new()
                || s.is_index_modified()
                || s.is_index_deleted()
                || s.is_index_renamed()
                || s.is_index_typechange() =>
            {
                1
            }
            _ => 0,
        };
    }

    if total_changes > 0 {
        let diff = repo.diff_index_to_workdir(None, None)?;
        let stats = diff.stats()?;
        Ok(Some(GitStats {
            insertions: stats.insertions() as u32,
            deletions: stats.deletions() as u32,
            total_changes: stats.files_changed() as u32,
        }))
    } else {
        Ok(None)
    }
}

fn send_notification(current_stats: Option<GitStats>, previous_stats: Option<GitStats>) {
    let body = uwu::get_notification_body(current_stats, previous_stats);

    Notification::new()
        .summary("Kwis :3")
        .body(&body)
        .show()
        .unwrap();
}
