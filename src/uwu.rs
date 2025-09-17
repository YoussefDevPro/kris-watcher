use crate::git::GitStats;

pub fn get_commit_message() -> &'static str {
    "this commit is made by kwis uwu"
}

pub fn get_notification_body(current_stats: Option<GitStats>, previous_stats: Option<GitStats>) -> String {
    match current_stats {
        Some(current) => {
            let mut message = String::new();

            if current.insertions == 0 && current.deletions == 0 {
                message.push_str("Nu-nu changes yet? owo Time to get to wowk!");
            } else {
                message.push_str(&"nyaa! ".to_string());

                if let Some(previous) = previous_stats {
                    if current.deletions > previous.deletions && current.deletions > 10 {
                        // deleting a lot for some reason
                        message.push_str(
                            "You'we on a deweting spwee! Cweaning up code wike a boss! ^w^",
                        );
                    } else if current.insertions > previous.insertions && current.insertions > 20 {
                        // becoming real ig
                        message.push_str(
                            "Wow, a sudden buwst of coding! Keep dat momentum going, uwu! ",
                        );
                    } else if current.deletions > current.insertions && current.deletions > 10 {
                        // peak optimishashtiong :3
                        message.push_str("Optimizing wike a pwo! Wess is mowe, wight? :3 ");
                    } else if current.insertions > current.deletions && current.insertions > 20 {
                        // too much insertion
                        message.push_str("Wook at chu, coding away! Keep it up, nyaa! ");
                    } else if current.total_changes > 30 && previous.total_changes < 5 {
                        // sudden tryharding
                        message
                            .push_str("You've been quiet, but now you'we a coding machine! UwU ");
                    }
                } else {
                    // peak optomozation
                    if current.deletions > current.insertions && current.deletions > 10 {
                        message.push_str("Optimizing wike a pwo! Wess is mowe, wight? :3 ");
                    } else if current.insertions > current.deletions && current.insertions > 20 {
                        message.push_str("Wook at chu, coding away! Keep it up, nyaa! ");
                    }
                }
                // do not forget to commit ur changes !!
                if message.is_empty() || !message.contains("commit") {
                    message.push_str("Don't fowget to commit youw changes, pwease! ^w^ ");
                }
            }
            message
        }
        None => "Don't fowget to commit youw changes, pwease! ^w^".to_string(),
    }
}
