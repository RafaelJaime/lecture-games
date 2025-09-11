use std::time::SystemTime;
use chrono::{DateTime, Local};

pub trait SystemTimeFormat {
    fn format_dm_yhm(&self) -> String;
}

impl SystemTimeFormat for SystemTime {
    fn format_dm_yhm(&self) -> String {
        let datetime: DateTime<Local> = (*self).into();
        datetime.format("%d/%m/%Y %H:%M").to_string()
    }
}