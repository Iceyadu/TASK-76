pub fn generate_time_slots(start_hour: u32, end_hour: u32, increment_min: u32) -> Vec<String> {
    let mut slots = Vec::new();
    let mut minutes = start_hour * 60;
    let end_minutes = end_hour * 60;
    while minutes < end_minutes {
        let h = minutes / 60;
        let m = minutes % 60;
        slots.push(format!("{:02}:{:02}", h, m));
        minutes += increment_min;
    }
    slots
}

pub fn round_to_15_min(minutes: u32) -> u32 {
    ((minutes + 7) / 15) * 15
}

pub fn is_within_business_hours(time: &str, start: &str, end: &str) -> bool {
    time >= start && time <= end
}
