use crate::tui::events::AnimationResult::{Commit, Quit};
use crossterm::event::{self, Event, KeyCode};
use std::error::Error;
use std::sync::mpsc::Sender;
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

pub fn handle_events(
    show_popup: &mut bool,
    popup_selection: &mut PopupSelection,
    reset_timer_tx: &Sender<()>,
) -> Result<Option<AnimationResult>, Box<dyn Error>> {
    if crossterm::event::poll(Duration::from_millis(10))? {
        if let Event::Key(key) = event::read()? {
            if *show_popup {
                match key.code {
                    KeyCode::Left | KeyCode::Char('y') => *popup_selection = PopupSelection::Yes,
                    KeyCode::Right | KeyCode::Char('n') => *popup_selection = PopupSelection::No,
                    KeyCode::Enter => match popup_selection {
                        PopupSelection::Yes => return Ok(Some(Commit)),
                        PopupSelection::No => {
                            *show_popup = false;
                            reset_timer_tx.send(()).ok();
                        }
                    },
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        *show_popup = false;
                        reset_timer_tx.send(()).ok();
                    }
                    _ => {}
                }
            } else if key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q') {
                return Ok(Some(Quit));
            }
        }
    }
    Ok(None)
}
