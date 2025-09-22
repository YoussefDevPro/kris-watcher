use std::time::Duration;

pub struct Config {
    pub autosave_mode: bool,
    pub loop_delay: Duration,
    pub shiggy_mode: bool,
    pub audio_alert_mode: bool,
}
//aa
impl Config {
    pub fn new() -> Result<Self, String> {
        let args: Vec<String> = std::env::args().collect();
        let autosave_mode = args.contains(&"--autosave".to_string());
        let shiggy_mode = args.contains(&"--shiggy".to_string());
        let audio_alert_mode = args.contains(&"-a".to_string());
        let mut loop_delay = Duration::from_secs(15 * 60);

        if let Some(pos) = args.iter().position(|s| s == "-l" || s == "--loop-delay") {
            if let Some(value_str) = args.get(pos + 1) {
                loop_delay = parse_duration(value_str).unwrap_or(loop_delay);
            }
        }

        Ok(Self {
            autosave_mode,
            loop_delay,
            shiggy_mode,
            audio_alert_mode,
        })
    }
}

fn parse_duration(s: &str) -> Result<Duration, String> {
    let s = s.trim();
    let mut numeric_part = String::new();
    let mut unit_part = String::new();

    for c in s.chars() {
        if c.is_digit(10) || c == '.' {
            numeric_part.push(c);
        } else {
            unit_part.push(c);
        }
    }

    let value: f64 = numeric_part
        .parse()
        .map_err(|_| "Invalid number".to_string())?;
    let unit = unit_part.trim();

    match unit {
        "s" | "sec" => Ok(Duration::from_secs_f64(value)),
        "m" | "min" => Ok(Duration::from_secs_f64(value * 60.0)),
        "h" => Ok(Duration::from_secs_f64(value * 3600.0)),
        _ => Err("Invalid time unit".to_string()),
    }
}
