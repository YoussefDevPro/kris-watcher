use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Notification {
    pub message: String,
    pub timestamp: Instant,
}

pub struct NotificationManager {
    notifications: VecDeque<Notification>,
    max_notifs: usize,
}

impl NotificationManager {
    pub fn new(max_notifs: usize) -> Self {
        Self {
            notifications: VecDeque::new(),
            max_notifs,
        }
    }

    pub fn add_notif(&mut self, message: String) {
        if self.notifications.len() == self.max_notifs {
            self.notifications.pop_front();
        }
        self.notifications.push_back(Notification {
            message,
            timestamp: Instant::now(),
        });
    }

    pub fn update(&mut self, max_age: Duration) {
        self.notifications
            .retain(|n| n.timestamp.elapsed() < max_age);
    }

    pub fn get_notifications(&self) -> &VecDeque<Notification> {
        &self.notifications
    }
}
